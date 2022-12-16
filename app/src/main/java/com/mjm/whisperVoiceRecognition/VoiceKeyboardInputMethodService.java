package com.mjm.whisperVoiceRecognition;

import android.content.Context;
import android.graphics.Bitmap;
import android.graphics.Color;
import android.inputmethodservice.InputMethodService;
import android.media.AudioDeviceInfo;
import android.media.AudioManager;
import android.view.View;
import android.widget.Button;
import android.widget.ToggleButton;

import com.example.WhisperVoiceKeyboard.R;

import java.io.FileOutputStream;
import java.io.IOException;
import java.util.Arrays;
import java.util.Optional;
import java.util.OptionalInt;

public class VoiceKeyboardInputMethodService extends InputMethodService {


    @Override
    public void onCreate() {
        super.onCreate();
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
                    float[] floatBuffer = byteBuffer.get();
                    //draw(floatBuffer);

                    getCurrentInputConnection().commitText("result", "result".length());

                }
            }
        });

        return inputView;
    }

    private void draw(float[] floatBuffer) {
        Bitmap bitmap = Bitmap.createBitmap(3000, 80, Bitmap.Config.ARGB_8888);

        //Loop through the float array and save each value as a pixel in the bitmap
        int x = 0;
        int y = 0;
        for (float f : floatBuffer) {
            int color = (int) (f * 255);
            int red = (int) (color * 0.5);
            int blue = 255 - red;
            bitmap.setPixel(y, x, Color.argb(255, red, 0, blue));
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


}