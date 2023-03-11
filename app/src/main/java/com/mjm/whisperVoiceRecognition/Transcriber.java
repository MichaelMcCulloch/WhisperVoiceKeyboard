package com.mjm.whisperVoiceRecognition;

import android.content.res.AssetFileDescriptor;
import android.content.res.AssetManager;
import android.util.Log;

import androidx.annotation.NonNull;

import org.tensorflow.lite.Interpreter;
import org.tensorflow.lite.flex.FlexDelegate;
import org.tensorflow.lite.gpu.GpuDelegate;
import org.tensorflow.lite.nnapi.NnApiDelegate;

import java.io.FileInputStream;
import java.io.IOException;
import java.nio.MappedByteBuffer;
import java.nio.channels.FileChannel;
import java.util.Arrays;
import java.util.HashMap;
import java.util.Map;

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


//        nnapiOptions.addDelegate(flexDelegate);
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

        int noTimestamps = _dictionary.getNotTimeStamps();
        int startOfTranscript = _dictionary.getStartOfTranscript();
        long[][] decoder_ids = new long[1][384];
        decoder_ids[0][0] = startOfTranscript;
        decoder_ids[0][1] = 50259; //+ lang;
        decoder_ids[0][2] = Vocab.TRANSCRIBE;
        decoder_ids[0][3] = noTimestamps;
        int prefixLen = 4;


        Map<String, Object> encoderInputsMap = new HashMap<String, Object>();
        String[] encoderInputs = _encoder.getSignatureInputs(SIGNATURE_KEY);
        encoderInputsMap.put(encoderInputs[0], reshape(byteBuffer, ENCODER_INPUT_SHAPE));

        Map<String, Object> encoderOutputsMap = new HashMap<String, Object>();
        String[] encoderOutputs = _encoder.getSignatureOutputs(SIGNATURE_KEY);
        encoderOutputsMap.put(encoderOutputs[0], encoderOutputBuffer);

        _encoder.runSignature(encoderInputsMap, encoderOutputsMap, SIGNATURE_KEY);


        Map<String, Object> decoderInputsMap = new HashMap<String, Object>();
        String[] decoderInputs = _decoder.getSignatureInputs(SIGNATURE_KEY);
        decoderInputsMap.put(decoderInputs[0], encoderOutputBuffer);
        decoderInputsMap.put(decoderInputs[1], decoder_ids);

        Map<String, Object> decoderOutputsMap = new HashMap<String, Object>();
        String[] decoderOutputs = _decoder.getSignatureOutputs(SIGNATURE_KEY);
        decoderOutputsMap.put(decoderOutputs[0], decoderOutputBuffer);

        int nextToken = -1;
        while (nextToken != _dictionary.getEndOfTranscript()) {
            _decoder.resizeInput(1, new int[]{1, prefixLen});
            _decoder.runSignature(decoderInputsMap, decoderOutputsMap, SIGNATURE_KEY);
            nextToken = maxTokenIndex(decoderOutputBuffer, prefixLen);

            decoder_ids[0][prefixLen] = nextToken;
            Log.i("transcribeAudio", "token: " + nextToken);
            Log.i("transcribeAudio", "token: " + Arrays.toString(decoder_ids[0]));
            prefixLen += 1;

        }

        long[] output = new long[prefixLen];
        System.arraycopy(decoder_ids[0], 0, output, 0, prefixLen);

//        _dictionary.logAllTokens();

        String whisperOutput = _dictionary.tokensToString(output);
        return _dictionary.injectTokens(whisperOutput);
    }

    private int maxTokenIndex(float[][][] decoderOutputBuffer, int index) {
        float[] sentence = decoderOutputBuffer[0][index];


        int lastTokenIndex = 0;
        float maxValue = Float.MIN_VALUE;
        for (int i = 0; i < sentence.length; i++) {
            if (sentence[i] > maxValue) {
                maxValue = sentence[i];
                lastTokenIndex = i;
            }
        }
        return lastTokenIndex;
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