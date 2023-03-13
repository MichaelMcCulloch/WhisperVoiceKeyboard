package com.mjm.whisperVoiceRecognition;

import android.content.res.AssetFileDescriptor;
import android.content.res.AssetManager;
import android.util.Log;

import androidx.annotation.NonNull;

import org.tensorflow.lite.Interpreter;
import org.tensorflow.lite.flex.FlexDelegate;

import java.io.FileInputStream;
import java.io.IOException;
import java.nio.MappedByteBuffer;
import java.nio.channels.FileChannel;
import java.util.HashMap;
import java.util.Map;

public class Transcriber {
    public static final String SIGNATURE_KEY = "serving_default";
    public static final String TAG = Transcriber.class.getName();
    public static final int[] ENCODER_INPUT_SHAPE = new int[]{1, 80, 3000};
    private static final String WHISPER_INTERPRETER = "nnmodel/mine/whisper-large-v2.tflite";
    private Interpreter _interpreter;
    private Dictionary _dictionary;

    public Transcriber(AssetManager assetManager) {
        Log.i(TAG, "transcribeAudio: Transcriber");
        Log.i(TAG, "transcribeAudio: Creating Options");
        Interpreter.Options nnapiOptions = new Interpreter.Options();
        Log.i(TAG, "transcribeAudio: Creating FlexDelegate");
        nnapiOptions.addDelegate(new FlexDelegate());
        Log.i(TAG, "transcribeAudio: Setting NumThreads");
        nnapiOptions.setNumThreads(8);
        Log.i(TAG, "transcribeAudio: Setting XNNPACK");
        nnapiOptions.setUseXNNPACK(true);
        Log.i(TAG, "transcribeAudio: Setting NNAPI");
        nnapiOptions.setUseNNAPI(false);

        try {
            Log.i(TAG, "transcribeAudio: Trying to load WhisperModel");
            MappedByteBuffer whisper_encoder = loadWhisperModel(assetManager, WHISPER_INTERPRETER);
            Log.i(TAG, "transcribeAudio: Creating Interpreter");
            _interpreter = new Interpreter(whisper_encoder, nnapiOptions);
            Log.i(TAG, "transcribeAudio: Extracting Vocab");
            Vocab vocab = ExtractVocab.extractVocab(assetManager.open("filters_vocab_multilingual.bin"));
            Log.i(TAG, "transcribeAudio: Creating phraseMappings");
            HashMap<String, String> phraseMappings = new HashMap<>();
            Log.i(TAG, "transcribeAudio: Creating Dictionary");
            _dictionary = new Dictionary(vocab, phraseMappings);
        } catch (Exception e) {
            Log.e(TAG, "transcribeAudio: Error loading WhisperModel", e);
            e.printStackTrace();
        }
    }

    @NonNull
    String transcribeAudio(float[] byteBuffer) {


        float[][][] reshapedFloats = reshapeInput(byteBuffer);
        int[][] tokenStream = new int[1][384];


        Map<String, Object> inferenceInputs = new HashMap<>();
        inferenceInputs.put("input_features", reshapedFloats);
        Map<String, Object> inferenceOutputs = new HashMap<>();
        inferenceOutputs.put("sequences", tokenStream);


        _interpreter.runSignature(inferenceInputs, inferenceOutputs, "infer_autodetect_transcribe_notimestamps");
        String whisperOutput = _dictionary.tokensToString(tokenStream);
        return _dictionary.injectTokens(whisperOutput);
    }

    @NonNull
    private float[][][] reshapeInput(float[] byteBuffer) {
        int[] shape = ENCODER_INPUT_SHAPE;
        float[][][] reshapedFloats = new float[shape[0]][shape[1]][shape[2]];
        int index = 0;
        for (int k = 0; k < shape[2]; k++) {
            for (int j = 0; j < shape[1]; j++) {
                for (int i = 0; i < shape[0]; i++) {
                    reshapedFloats[i][j][k] = byteBuffer[index];
                    index++;
                }
            }
        }
        return reshapedFloats;
    }

    private static MappedByteBuffer loadWhisperModel(AssetManager assets, String modelName)
            throws IOException {
        AssetFileDescriptor fileDescriptor = assets.openFd(modelName);
        FileChannel fileChannel;
        try (FileInputStream inputStream = new FileInputStream(fileDescriptor.getFileDescriptor())) {
            fileChannel = inputStream.getChannel();
            long startOffset = fileDescriptor.getStartOffset();
            long declaredLength = fileDescriptor.getDeclaredLength();
            return fileChannel.map(FileChannel.MapMode.READ_ONLY, startOffset, declaredLength);
        }

    }
}