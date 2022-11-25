package com.example.whisperVoiceRecognition;

import android.content.Context;
import android.inputmethodservice.InputMethodService;
import android.media.AudioDeviceInfo;
import android.media.AudioManager;
import android.util.Log;
import android.view.View;
import android.widget.ToggleButton;

import com.example.WhisperVoiceKeyboard.R;

import java.util.Arrays;
import java.util.Optional;
import java.util.OptionalInt;

public class VoiceKeyboardInputMethodService extends InputMethodService {

    @Override
    public void onCreate() {
        super.onCreate();
        System.loadLibrary("rust");
        initializeLibraryAudioConfig();
    }

    private void initializeLibraryAudioConfig() {
        AudioManager audioManager = (AudioManager) getSystemService(Context.AUDIO_SERVICE);
        if (android.os.Build.VERSION.SDK_INT >= android.os.Build.VERSION_CODES.M) {
            AudioDeviceInfo[] adi = audioManager.getDevices(AudioManager.GET_DEVICES_INPUTS);
            Log.i("VoiceKeyboardInputMethodService", "Number of inputs: " + adi.length);
            Optional<AudioDeviceInfo> bottomMic = Arrays.stream(adi).map(x -> {
                Log.i("VoiceKeyboardInputMethodService", "Placement: " + x.getAddress());
                return x;
            }).filter(audioDeviceInfo -> audioDeviceInfo.getAddress().equals("bottom")).findAny();


            if (bottomMic.isPresent()) {
                OptionalInt sampleRate = Arrays.stream(bottomMic.get().getSampleRates()).max();
                OptionalInt channels = Arrays.stream(bottomMic.get().getChannelCounts()).min();
                if (sampleRate.isPresent()){
                    RustLib.init(getApplicationContext(), bottomMic.get().getId(), sampleRate.getAsInt(),channels.getAsInt());

                }

            }

        }
    }


    @Override
    public void onDestroy() {
        RustLib.uninit();
        super.onDestroy();
    }

    WhisperVoiceTranscriptionDriver driver = new WhisperVoiceTranscriptionDriver();

    @Override
    public View onCreateInputView() {
        View inputView =
                getLayoutInflater().inflate(R.layout.keyboard, null);
        ToggleButton recordButton = inputView.findViewById(R.id.buttonRecord);

        recordButton.setOnCheckedChangeListener((button,checked) -> {
            if (checked) {
                RustLib r = new RustLib();
                r.sampleAudio();
                startVoiceService();
            } else {
                stopVoiceService();
            }
        });

        return inputView;
    }



    private void startVoiceService() {
        Log.i("VoiceKeyboardService","Init Service");
        driver.startListening();

    }

    private void stopVoiceService() {
        Log.i("VoiceKeyboardService","Term Service");
        String text = driver.stopListeningAndRetrieveText();
        getCurrentInputConnection().commitText(text, text.length());

    }

}