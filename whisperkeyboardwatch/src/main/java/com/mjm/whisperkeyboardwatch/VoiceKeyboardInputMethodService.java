package com.mjm.whisperkeyboardwatch;

import static android.Manifest.permission.RECORD_AUDIO;

import android.annotation.SuppressLint;
import android.content.Context;
import android.content.Intent;
import android.content.pm.PackageManager;
import android.content.res.AssetFileDescriptor;
import android.content.res.AssetManager;
import android.inputmethodservice.InputMethodService;
import android.media.AudioDeviceInfo;
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

import androidx.annotation.NonNull;
import androidx.constraintlayout.widget.ConstraintLayout;

import org.tensorflow.lite.Interpreter;

import java.io.FileInputStream;
import java.io.IOException;
import java.nio.MappedByteBuffer;
import java.nio.channels.FileChannel;
import java.util.Arrays;
import java.util.HashMap;
import java.util.Map;
import java.util.Optional;
import java.util.OptionalInt;

public class VoiceKeyboardInputMethodService extends InputMethodService {

    private Interpreter _whisperInterpreter;
    private Dictionary _dictionary;

    private static final String WHISPER_TFLITE = "whisper.tflite";

    private static final boolean LOG_AND_DRAW = false;


    @Override
    public void onCreate() {

        super.onCreate();


        try {
            Vocab vocab = ExtractVocab.extractVocab(getAssets().open("filters_vocab_gen.bin"));
            HashMap<String, String> phraseMappings = new HashMap<>();


            _dictionary = new Dictionary(vocab, phraseMappings);
            MappedByteBuffer model = loadWhisperModel(getAssets());

            Interpreter.Options options = new Interpreter.Options();

            options.setUseXNNPACK(true);
            options.setNumThreads(8);

            _whisperInterpreter = new Interpreter(model, options);


        } catch (IOException e) {
            e.printStackTrace();
        }
        RustLib.init(getAssets());
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

        recordButton.setOnCheckedChangeListener((button, checked) -> {
            if (checked && getBottomMicrophone().isPresent()) {
                RustLib.startRecording(getBottomMicrophone().get());
                cancelButton.setVisibility(View.VISIBLE);
            } else {
                cancelButton.setVisibility(View.GONE);
                Optional<float[]> byteBuffer = RustLib.endRec();
                if (byteBuffer.isPresent()) {
                    String transcribeAudio = transcribeAudio(byteBuffer.get());
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


    @NonNull
    private String transcribeAudio(float[] byteBuffer) {
        int[] inputShape = {1, 80, 3000};

        float[][][] reshapedFloats = reshapeInput(byteBuffer, inputShape);
        int[][] output = new int[1][224];

        Map<String, Object> inputs = new HashMap<>();
        inputs.put("input_features", reshapedFloats);
        Map<String, Object> outputs = new HashMap<>();
        outputs.put("sequences", output);

        _whisperInterpreter.runSignature(inputs, outputs, "serving_default");
        String whisperOutput = _dictionary.tokensToString(output);
        return _dictionary.injectTokens(whisperOutput);
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
        AssetFileDescriptor fileDescriptor = assets.openFd(WHISPER_TFLITE);
        FileInputStream inputStream = new FileInputStream(fileDescriptor.getFileDescriptor());
        FileChannel fileChannel = inputStream.getChannel();
        long startOffset = fileDescriptor.getStartOffset();
        long declaredLength = fileDescriptor.getDeclaredLength();
        return fileChannel.map(FileChannel.MapMode.READ_ONLY, startOffset, declaredLength);
    }

}