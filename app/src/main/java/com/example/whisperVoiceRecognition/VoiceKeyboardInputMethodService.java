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

    WhisperVoiceTranscriptionDriver driver = new WhisperVoiceTranscriptionDriver();

    @Override
    public View onCreateInputView() {
        View inputView =
                getLayoutInflater().inflate(R.layout.keyboard, null);
        ToggleButton recordButton = inputView.findViewById(R.id.buttonRecord);




        recordButton.setOnCheckedChangeListener((button,checked) -> {
            if (checked) {
//                ActivityCompat.requestPermissions (VoiceKeyboardInputMethodService.this, new String[]{Manifest.permission.RECORD_AUDIO},
//                        REQUEST_RECORD_PERMISSION);
//                Log.d("Rust FFI", new RustLib().helloWorld());


                AssetManager assetManager = getAssets();
                try {
                    InputStream is = assetManager.open("whisper.tflite");
                    Log.d("VoiceKeyboardInputMethodService", ""+is.available());
                } catch (IOException e) {
                    e.printStackTrace();
                }
                RustLib rustLib = new RustLib();
                rustLib.init(getBaseContext());
                rustLib.retrieveAssetPub(assetManager);
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