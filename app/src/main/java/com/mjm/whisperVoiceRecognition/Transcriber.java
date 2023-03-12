package com.mjm.whisperVoiceRecognition;

import android.content.res.AssetFileDescriptor;
import android.content.res.AssetManager;

import androidx.annotation.NonNull;

import org.tensorflow.lite.Interpreter;
import org.tensorflow.lite.flex.FlexDelegate;
import org.tensorflow.lite.gpu.GpuDelegate;
import org.tensorflow.lite.nnapi.NnApiDelegate;

import java.io.FileInputStream;
import java.io.IOException;
import java.nio.MappedByteBuffer;
import java.nio.channels.FileChannel;
import java.util.HashMap;
import java.util.Map;
import java.util.Vector;

public class Transcriber {
    public static final String SIGNATURE_KEY = "serving_default";
    public static final int[] ENCODER_INPUT_SHAPE = new int[]{1, 80, 3000};
    private static final String WHISPER_ENCODER = "nnmodel/mine/whisper-encoder.tflite";
    private static final String WHISPER_DECODER_LANGUAGE = "nnmodel/mine/whisper-decoder_language.tflite";
    private Interpreter _encoder;
    private Interpreter _decoder;
    private Dictionary _dictionary;

    public Transcriber(AssetManager assetManager) {


        Interpreter.Options nnapiOptions = new Interpreter.Options();
        NnApiDelegate nnapiDelegate = new NnApiDelegate();
        FlexDelegate flexDelegate = new FlexDelegate();
        GpuDelegate gpuDelegate = new GpuDelegate();


        nnapiOptions.addDelegate(flexDelegate);
//        nnapiOptions.addDelegate(gpuDelegate);
//        nnapiOptions.addDelegate(nnapiDelegate);


        nnapiOptions.setNumThreads(8);
        nnapiOptions.setUseXNNPACK(true);
        nnapiOptions.setUseNNAPI(false);

        try {


            MappedByteBuffer whisper_encoder = loadWhisperModel(assetManager, WHISPER_ENCODER);
            MappedByteBuffer whisper_decoder_language = loadWhisperModel(assetManager, WHISPER_DECODER_LANGUAGE);

            _encoder = new Interpreter(whisper_encoder, nnapiOptions);
            _decoder = new Interpreter(whisper_decoder_language, nnapiOptions);
            Vocab vocab = ExtractVocab.extractVocab(assetManager.open("filters_vocab_multilingual.bin"));
            HashMap<String, String> phraseMappings = new HashMap<>();
            _dictionary = new Dictionary(vocab, phraseMappings);

        } catch (Exception e) {
            e.printStackTrace();
        }

    }

    @NonNull
    String transcribeAudio(float[] byteBuffer) {

        float[][][] encoderOutputBuffer = new float[1][1500][384];
        float[][][] decoderOutputBuffer = new float[1][384][51865];

        long[][] decoderInputIds = new long[1][384];
        long[] prefix = {_dictionary.getStartOfTranscript(), 50259, Vocab.TRANSLATE, _dictionary.getNotTimeStamps()};
        int prefixLen = prefix.length;
        System.arraycopy(prefix, 0, decoderInputIds[0], 0, prefixLen);

        Vector<Long> tokenStream = new Vector<>(4);
        for (int p = 0; p < prefixLen; p++) {
            tokenStream.add(prefix[p]);
        }

        Map<String, Object> encoderInputsMap = new HashMap<>();
        String[] encoderInputs = _encoder.getSignatureInputs(SIGNATURE_KEY);
        encoderInputsMap.put(encoderInputs[0], reshape(byteBuffer, ENCODER_INPUT_SHAPE));

        Map<String, Object> encoderOutputsMap = new HashMap<>();
        String[] encoderOutputs = _encoder.getSignatureOutputs(SIGNATURE_KEY);
        encoderOutputsMap.put(encoderOutputs[0], encoderOutputBuffer);

        _encoder.runSignature(encoderInputsMap, encoderOutputsMap, SIGNATURE_KEY);


        Map<String, Object> decoderInputsMap = new HashMap<>();
        String[] decoderInputs = _decoder.getSignatureInputs(SIGNATURE_KEY);
        decoderInputsMap.put(decoderInputs[0], encoderOutputBuffer);
        decoderInputsMap.put(decoderInputs[1], decoderInputIds);

        Map<String, Object> decoderOutputsMap = new HashMap<>();
        String[] decoderOutputs = _decoder.getSignatureOutputs(SIGNATURE_KEY);
        decoderOutputsMap.put(decoderOutputs[0], decoderOutputBuffer);

        int nextToken = -1;
        while (nextToken != _dictionary.getEndOfTranscript()) {
            _decoder.resizeInput(1, new int[]{1, prefixLen});
            _decoder.runSignature(decoderInputsMap, decoderOutputsMap, SIGNATURE_KEY);
            nextToken = argmax(decoderOutputBuffer[0], prefixLen - 1);
            tokenStream.add((long) nextToken);
            decoderInputIds[0][prefixLen] = nextToken;

            prefixLen += 1;

        }

        String whisperOutput = _dictionary.tokensToString(tokenStream);
        return _dictionary.injectTokens(whisperOutput);
    }

    private int argmax(float[][] decoderOutputBuffer, int index) {

        int maxIndex = 0;
        for (int j = 0; j < decoderOutputBuffer[index].length; j++) {
            if (decoderOutputBuffer[index][j] > decoderOutputBuffer[index][maxIndex]) {
                maxIndex = j;
            }
        }

        return maxIndex;

    }

    private static MappedByteBuffer loadWhisperModel(AssetManager assets, String modelName)
            throws IOException {
        AssetFileDescriptor fileDescriptor = assets.openFd(modelName);
        FileInputStream inputStream = new FileInputStream(fileDescriptor.getFileDescriptor());
        FileChannel fileChannel = inputStream.getChannel();
        long startOffset = fileDescriptor.getStartOffset();
        long declaredLength = fileDescriptor.getDeclaredLength();
        return fileChannel.map(FileChannel.MapMode.READ_ONLY, startOffset, declaredLength);
    }

    @NonNull
    private float[][][] reshape(float[] byteBuffer, int[] inputShape) {
        float[][][] reshapedFloats = new float[inputShape[0]][inputShape[1]][inputShape[2]];
        int index = 0;
        for (int k = 0; k < inputShape[2]; k++) {
            for (int j = 0; j < inputShape[1]; j++) {
                for (int i = 0; i < inputShape[0]; i++) {
                    reshapedFloats[i][j][k] = byteBuffer[index];
                    index++;
                }
            }
        }
        return reshapedFloats;
    }
}