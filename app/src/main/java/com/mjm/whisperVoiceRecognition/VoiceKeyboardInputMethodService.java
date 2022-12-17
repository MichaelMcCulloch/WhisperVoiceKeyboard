package com.mjm.whisperVoiceRecognition;

import android.content.Context;
import android.content.res.AssetFileDescriptor;
import android.content.res.AssetManager;
import android.graphics.Bitmap;
import android.graphics.Color;
import android.inputmethodservice.InputMethodService;
import android.media.AudioDeviceInfo;
import android.media.AudioManager;
import android.util.Log;
import android.view.View;
import android.widget.Button;
import android.widget.ToggleButton;

import androidx.annotation.NonNull;

import com.example.WhisperVoiceKeyboard.R;

import org.tensorflow.lite.Interpreter;

import java.io.FileInputStream;
import java.io.FileOutputStream;
import java.io.IOException;
import java.nio.MappedByteBuffer;
import java.nio.channels.FileChannel;
import java.util.Arrays;
import java.util.HashMap;
import java.util.Map;
import java.util.Optional;
import java.util.OptionalInt;

public class VoiceKeyboardInputMethodService extends InputMethodService {

    private static Vocab _vocab;
    private static Interpreter _whisperInterpreter;

    @Override
    public void onCreate() {
        super.onCreate();
        try {
            _vocab = ExtractVocab.extractVocab(getAssets().open("filters_vocab_gen.bin"));
            MappedByteBuffer model = loadWhisperModel(getAssets());
            Log.i("TFLITE", "onCreateInputView: " + "Created tflitemodel");
            _whisperInterpreter = new Interpreter(model);


        } catch (IOException e) {
            e.printStackTrace();
        }
        RustLib.init(getAssets());

    }

    @Override
    public void onDestroy() {
        RustLib.uninit();
        super.onDestroy();
    }

    @Override
    public View onCreateInputView() {
        View inputView =
                getLayoutInflater().inflate(R.layout.keyboard, null);
        ToggleButton recordButton = inputView.findViewById(R.id.buttonRecord);
        Button cancelButton = inputView.findViewById(R.id.buttonCancel);


        cancelButton.setOnClickListener(v -> {
            RustLib.abortRecording();
            recordButton.setChecked(false);

        });

        recordButton.setOnCheckedChangeListener((button, checked) -> {
            if (checked && getBottomMicrophone().isPresent()) {
                RustLib.startRecording(getBottomMicrophone().get());

            } else {
                Optional<float[]> byteBuffer = RustLib.endRec();
                if (byteBuffer.isPresent()) {
                    String transcribed = transcribAudio(byteBuffer.get()).trim() + " ";
                    getCurrentInputConnection().commitText(transcribed, transcribed.length());

                }
            }
        });

        return inputView;
    }

    @NonNull
    private String transcribAudio(float[] byteBuffer) {
        int[] inputShape = {1, 80, 3000};

        float[][][] reshapedFloats = reshapeInput(byteBuffer, inputShape);
        int[][] output = new int[1][224];

        Map<String, Object> inputs = new HashMap<>();
        inputs.put("input_features", reshapedFloats);
        Map<String, Object> outputs = new HashMap<>();
        outputs.put("sequences", output);
        _whisperInterpreter.runSignature(inputs, outputs, "serving_default");

        String transcribed = tokensToString(output);
        return transcribed;
    }

    @NonNull
    private String tokensToString(int[][] output) {
        StringBuilder sb = new StringBuilder();
        for (int token : output[0]) {
            if (token == _vocab.token_eot) {
                break;
            }
            if (token != 50257 && token != 50362) {
                String word = _vocab.id_to_token.get(token);
                Log.i("tokenization", "token: " + token + " word " + word);
                sb.append(word);
            }
        }
        String transcribed = sb.toString();
        return transcribed;
    }

    @NonNull
    private float[][][] reshapeInput(float[] byteBuffer, int[] inputShape) {
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

    private void draw(float[] floats) {


        Bitmap bitmap = Bitmap.createBitmap(3000, 80, Bitmap.Config.ARGB_8888);

        //Loop through the float array and save each value as a pixel in the bitmap
        int x = 0;
        int y = 0;
        for (float f : floats) {
            int color = (int) ((f + 1) / 2 * 255);
            bitmap.setPixel(y, x, Color.argb(255, color, color, color));
            x++;
            if (x >= 80) {
                x = 0;
                y++;
            }
        }
        //write the bitmap to file
        try {
            FileOutputStream out = new FileOutputStream(getFilesDir().getAbsolutePath() + "/spectrogram3.png");
            bitmap.compress(Bitmap.CompressFormat.PNG, 100, out);
            out.close();
        } catch (IOException e) {
            e.printStackTrace();
        }
    }


    private Optional<AudioDeviceConfig> getBottomMicrophone() {
        AudioManager audioManager = (AudioManager) getSystemService(Context.AUDIO_SERVICE);
        AudioDeviceInfo[] adi = audioManager.getDevices(AudioManager.GET_DEVICES_INPUTS);
        Optional<AudioDeviceInfo> bottomMic = Arrays.stream(adi)
                .filter(audioDeviceInfo -> audioDeviceInfo.getAddress().equals("bottom"))
                .findAny();

        if (bottomMic.isPresent()) {

            OptionalInt maxSampleRate = Arrays.stream(bottomMic.get().getSampleRates())
                    .max();
            OptionalInt minChannels = Arrays.stream(bottomMic.get().getChannelCounts())
                    .min();
            if (maxSampleRate.isPresent() && minChannels.isPresent()) {
                AudioDeviceConfig audioDeviceConfig = new AudioDeviceConfig(bottomMic.get().getId(), maxSampleRate.getAsInt(), minChannels.getAsInt());

                return Optional.of(audioDeviceConfig);
            }
            return Optional.empty();
        }
        return Optional.empty();
    }

    private static MappedByteBuffer loadWhisperModel(AssetManager assets)
            throws IOException {
        AssetFileDescriptor fileDescriptor = assets.openFd("whisper.tflite");
        FileInputStream inputStream = new FileInputStream(fileDescriptor.getFileDescriptor());
        FileChannel fileChannel = inputStream.getChannel();
        long startOffset = fileDescriptor.getStartOffset();
        long declaredLength = fileDescriptor.getDeclaredLength();
        return fileChannel.map(FileChannel.MapMode.READ_ONLY, startOffset, declaredLength);
    }

}