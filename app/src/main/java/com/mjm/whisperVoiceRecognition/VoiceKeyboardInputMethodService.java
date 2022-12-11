package com.mjm.whisperVoiceRecognition;

import android.content.Context;
import android.inputmethodservice.InputMethodService;
import android.media.AudioDeviceInfo;
import android.media.AudioManager;
import android.util.Log;
import android.view.View;
import android.widget.Button;
import android.widget.ToggleButton;

import com.example.WhisperVoiceKeyboard.R;

import java.nio.ByteBuffer;
import java.util.Arrays;
import java.util.Optional;
import java.util.OptionalInt;

public class VoiceKeyboardInputMethodService extends InputMethodService {

    
    private boolean recording = false;

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
            if (recording) {
                RustLib.abortRecording();
                recording = false;
                recordButton.setChecked(false);
            }

        });

        recordButton.setOnCheckedChangeListener((button, checked) -> {
            if (checked && getBottomMicrophone().isPresent()) {

                recording = true;
                RustLib.startRecording(getBottomMicrophone().get());
            } else if (recording) {
                Log.i("TAG", "endRec: " + getApplicationContext().getPackageName());

                Optional<ByteBuffer> byteBuffer = RustLib.endRec();
                if (byteBuffer.isPresent()) {
                    getCurrentInputConnection().commitText("result", "result".length());

                }
                recording = false;
            }
        });

        return inputView;
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