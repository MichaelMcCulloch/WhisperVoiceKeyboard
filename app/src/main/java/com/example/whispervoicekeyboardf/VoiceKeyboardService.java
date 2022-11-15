package com.example.whispervoicekeyboardf;


import android.inputmethodservice.InputMethodService;
import android.view.View;

import com.example.WhisperVoiceKeyboard.R;

public class VoiceKeyboardService extends InputMethodService {

    @Override
    public View onCreateInputView() {
        View inputView =
               getLayoutInflater().inflate(R.layout.keyboard, null);


        return inputView;
    }

}
