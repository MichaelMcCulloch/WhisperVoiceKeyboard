package com.example.whisperVoiceRecognition;

import android.content.Context;
import android.inputmethodservice.InputMethodService;
import android.media.AudioDeviceInfo;
import android.media.AudioManager;
import android.util.Log;
import android.view.View;
import android.widget.ToggleButton;

import com.example.WhisperVoiceKeyboard.R;

import java.io.File;
import java.io.FileOutputStream;
import java.io.IOException;
import java.io.InputStream;
import java.util.Arrays;
import java.util.Optional;
import java.util.OptionalInt;

public class VoiceKeyboardInputMethodService extends InputMethodService {

    @Override
    public void onCreate() {
        super.onCreate();
        System.loadLibrary("rust");

        try {
            InputStream is = getAssets().open("whisper.tflite");

            byte[] content = new byte[is.available()];

            int _read = is.read(content);


            File recording = new File(getCacheDir(), "whisper.tflite");
            FileOutputStream fos = new FileOutputStream(recording);
            fos.write(content);

            fos.close();
            String path = recording.getAbsolutePath();
            Log.i("VoiceKeyboardInputMethodService", "onCreate: " + path);


            RustLib.init(getApplicationContext(), getAssets(), path);

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

        recordButton.setOnCheckedChangeListener((button, checked) -> {
            if (checked && getBottomMicrophone().isPresent()) {
                AudioDeviceConfig bottomMic = getBottomMicrophone().get();
                RustLib.startRecording(bottomMic.getDeviceId(), bottomMic.getDeviceSampleRate(), bottomMic.getDeviceChannels());
            } else {
                String result = RustLib.endRecording();
                getCurrentInputConnection().commitText(result, result.length());
            }
        });

        return inputView;
    }


}