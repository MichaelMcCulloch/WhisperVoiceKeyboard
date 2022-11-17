package com.example.whisperVoiceRecognition;

import android.content.res.AssetManager;
import android.inputmethodservice.InputMethodService;
import android.util.Log;
import android.view.View;
import android.widget.ToggleButton;

import com.example.WhisperVoiceKeyboard.R;

import java.io.IOException;
import java.io.InputStream;

public class VoiceKeyboardInputMethodService extends InputMethodService {

    @Override
    public void onCreate() {
        super.onCreate();
        System.loadLibrary("rust");
        RustLib.init(getBaseContext());
    }

    @Override
    public void onDestroy() {
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
                r.retrieveAssetPub(getAssets());
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