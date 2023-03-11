package com.mjm.whisperVoiceRecognition;

import static android.Manifest.permission.RECORD_AUDIO;

import android.annotation.SuppressLint;
import android.content.Context;
import android.content.Intent;
import android.content.pm.PackageManager;
import android.content.res.AssetManager;
import android.inputmethodservice.InputMethodService;
import android.media.AudioManager;
import android.os.Handler;
import android.os.Looper;
import android.view.ContextThemeWrapper;
import android.view.KeyEvent;
import android.view.LayoutInflater;
import android.view.MotionEvent;
import android.view.View;
import android.view.inputmethod.EditorInfo;
import android.widget.Button;
import android.widget.ImageButton;
import android.widget.Toast;
import android.widget.ToggleButton;

import androidx.constraintlayout.widget.ConstraintLayout;

import com.example.WhisperVoiceKeyboard.R;

import java.util.Optional;

public class VoiceKeyboardInputMethodService extends InputMethodService {

    private Transcriber _transcriber;


    private static final boolean LOG_AND_DRAW = false;


    @Override
    public void onCreate() {
        super.onCreate();

        AssetManager assetManager = getAssets();

        RustLib.init(assetManager);

        _transcriber = new Transcriber(assetManager);
    }


    @Override
    public void onDestroy() {
        super.onDestroy();
        RustLib.uninit();

    }

    @Override
    public void onStartInputView(EditorInfo info, boolean restarting) {
        super.onStartInputView(info, restarting);
        if (PackageManager.PERMISSION_GRANTED != checkSelfPermission(RECORD_AUDIO)) {
            Toast.makeText(this, R.string.toast_grant_microphone_permission, Toast.LENGTH_LONG).show();
            Intent intent = new Intent(this, Wizard.class);
            intent.addFlags(Intent.FLAG_ACTIVITY_NEW_TASK);
            startActivity(intent);
        }
    }

    @SuppressLint({"ClickableViewAccessibility", "InflateParams"})
    @Override
    public View onCreateInputView() {
        View inputView;
        ContextThemeWrapper ctx = new ContextThemeWrapper(this, R.style.Theme_WhisperVoiceKeyboard);

        inputView = (ConstraintLayout) LayoutInflater.from(ctx).inflate(R.layout.keyboard, null);
        ToggleButton recordButton = inputView.findViewById(R.id.buttonRecord);
        Button cancelButton = inputView.findViewById(R.id.buttonCancel);
        ImageButton deleteButton = inputView.findViewById(R.id.buttonDelete);
        ImageButton newlineButton = inputView.findViewById(R.id.buttonNewline);


        newlineButton.setOnClickListener(view -> {
            getCurrentInputConnection().sendKeyEvent(new KeyEvent(KeyEvent.ACTION_DOWN, KeyEvent.KEYCODE_ENTER));
            getCurrentInputConnection().sendKeyEvent(new KeyEvent(KeyEvent.ACTION_UP, KeyEvent.KEYCODE_ENTER));
        });

        deleteButton.setOnClickListener(view -> {
            sendDelete();
        });

        deleteButton.setOnTouchListener(new View.OnTouchListener() {
            private Handler mHandler;
            final Runnable mAction = new Runnable() {
                @Override
                public void run() {
                    sendDelete();
                    mHandler.postDelayed(this, 100);
                }
            };

            @Override
            public boolean onTouch(View view, MotionEvent motionEvent) {
                switch (motionEvent.getAction()) {
                    case MotionEvent.ACTION_DOWN:
                        if (mHandler != null) return true;
                        mHandler = new Handler(Looper.getMainLooper());
                        mHandler.post(mAction);
                        return true;
                    case MotionEvent.ACTION_UP:
                    case MotionEvent.ACTION_CANCEL:
                        if (mHandler == null) return true;
                        mHandler.removeCallbacks(mAction);
                        mHandler = null;
                        return true;
                }
                return false;
            }
        });

        cancelButton.setOnClickListener(v -> {
            RustLib.abortRecording();
            recordButton.setChecked(false);
            cancelButton.setVisibility(View.GONE);
        });

        final MicrophoneResolver microphoneResolver = new MicrophoneResolver((AudioManager) getSystemService(Context.AUDIO_SERVICE));

        recordButton.setOnCheckedChangeListener((button, checked) -> {
            Optional<AudioDeviceConfig> microphone = microphoneResolver.resolveMicrophone();
            if (checked && microphone.isPresent()) {
                AudioDeviceConfig foundMicrophone = microphone.get();
                RustLib.startRecording(foundMicrophone);
                cancelButton.setVisibility(View.VISIBLE);
            } else {
                cancelButton.setVisibility(View.GONE);
                Optional<float[]> byteBuffer = RustLib.endRec();
                if (byteBuffer.isPresent()) {
                    String transcribeAudio = _transcriber.transcribeAudio(byteBuffer.get());
                    String transcribed = transcribeAudio.trim() + " ";
                    getCurrentInputConnection().commitText(transcribed, 1);
                    if (LOG_AND_DRAW) {
                        SpectrogramToFile.save(byteBuffer.get(), getFilesDir().getAbsolutePath());
                    }
                }
            }
        });

        return inputView;
    }


    private void sendDelete() {
        getCurrentInputConnection().sendKeyEvent(new KeyEvent(KeyEvent.ACTION_DOWN, KeyEvent.KEYCODE_DEL));
        getCurrentInputConnection().sendKeyEvent(new KeyEvent(KeyEvent.ACTION_UP, KeyEvent.KEYCODE_DEL));
    }


}