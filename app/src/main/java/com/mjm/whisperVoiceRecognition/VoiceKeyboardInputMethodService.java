package com.mjm.whisperVoiceRecognition;

import static android.Manifest.permission.RECORD_AUDIO;

import android.annotation.SuppressLint;
import android.content.Context;
import android.content.Intent;
import android.content.pm.PackageManager;
import android.content.res.AssetFileDescriptor;
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

import androidx.annotation.NonNull;
import androidx.constraintlayout.widget.ConstraintLayout;

import com.example.WhisperVoiceKeyboard.R;

import org.tensorflow.lite.Interpreter;
import org.tensorflow.lite.flex.FlexDelegate;
import org.tensorflow.lite.gpu.GpuDelegate;
import org.tensorflow.lite.nnapi.NnApiDelegate;

import java.io.FileInputStream;
import java.io.IOException;
import java.nio.MappedByteBuffer;
import java.nio.channels.FileChannel;
import java.util.HashMap;
import java.util.Map;
import java.util.Optional;

public class VoiceKeyboardInputMethodService extends InputMethodService {

    private Interpreter _nnapiEncoder;
    private Interpreter _nnapiDecoder;
    private Dictionary _dictionary;


    private static final String WHISPER_ENCODER = "nnmodel/nyadia/whisper-encoder.tflite";
    private static final String WHISPER_DECODER_LANGUAGE = "nnmodel/nyadia/whisper-decoder_language.tflite";

    private static final boolean LOG_AND_DRAW = false;


    @Override
    public void onCreate() {

        super.onCreate();


        Interpreter.Options nnapiOptions = new Interpreter.Options();
        NnApiDelegate nnapiDelegate = new NnApiDelegate();
        FlexDelegate flexDelegate = new FlexDelegate();
        GpuDelegate gpuDelegate = new GpuDelegate();


        nnapiOptions.addDelegate(flexDelegate);
        nnapiOptions.addDelegate(gpuDelegate);
        nnapiOptions.addDelegate(nnapiDelegate);


        nnapiOptions.setNumThreads(0);
        nnapiOptions.setUseXNNPACK(true);
        nnapiOptions.setUseNNAPI(true);

        try {


            MappedByteBuffer whisper_encoder = loadWhisperModel(getAssets(), WHISPER_ENCODER);
            MappedByteBuffer whisper_decoder_language = loadWhisperModel(getAssets(), WHISPER_DECODER_LANGUAGE);

            _nnapiEncoder = new Interpreter(whisper_encoder, nnapiOptions);
            _nnapiDecoder = new Interpreter(whisper_decoder_language, nnapiOptions);

            Vocab vocab = ExtractVocab.extractVocab(getAssets().open("filters_vocab_gen.bin"));
            HashMap<String, String> phraseMappings = new HashMap<>();
            _dictionary = new Dictionary(vocab, phraseMappings);

        } catch (Exception e) {
            e.printStackTrace();
            System.exit(-1);
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

        Map<String, Object> inputsEncoder = new HashMap<>();
        Map<String, Object> outputsEncoder = new HashMap<>();
        Map<String, Object> inputsDecoder = new HashMap<>();
        Map<String, Object> outputsDecoder = new HashMap<>();

        String signatureKey = "serving_default";
        String[] nnapiEncoderSignatureInputs = _nnapiEncoder.getSignatureInputs(signatureKey);
        String[] nnapiEncoderSignatureOutputs = _nnapiEncoder.getSignatureOutputs(signatureKey);
        String[] nnapiDecoderSignatureInputs = _nnapiDecoder.getSignatureInputs(signatureKey);
        String[] nnapiDecoderSignatureOutputs = _nnapiDecoder.getSignatureOutputs(signatureKey);

        String encoderInputKey0 = nnapiEncoderSignatureInputs[0];
        String encoderOutputKey0 = nnapiEncoderSignatureOutputs[0];
        String decoderInputKey0 = nnapiDecoderSignatureInputs[0];
        String decoderInputKey1 = nnapiDecoderSignatureInputs[1];
        String decoderOutputKey0 = nnapiDecoderSignatureOutputs[0];

        inputsEncoder.put(encoderInputKey0, reshapeInput(byteBuffer, inputShape));
        float[][][] encoder_output = new float[1][1500][384];
        outputsEncoder.put(encoderOutputKey0, encoder_output);


        _nnapiEncoder.runSignature(inputsEncoder, outputsEncoder, signatureKey);


        long[][][] encoder_output_int = new long[1][1500][384];

        inputsDecoder.put(decoderInputKey0, encoder_output_int);

        float[][] decoder_ids = new float[1][384];
        decoder_ids[0][0] = 50258;
        decoder_ids[0][1] = 50266;
        decoder_ids[0][2] = 50358;
        decoder_ids[0][3] = 50363;
        inputsDecoder.put(decoderInputKey1, decoder_ids);

        int[] shape = new int[2];
        shape[0] = 1;
        shape[1] = 4;
        _nnapiDecoder.resizeInput(1, shape);
        float[][] output = new float[1][224];
        outputsDecoder.put(decoderOutputKey0, output);

        _nnapiDecoder.runSignature(inputsDecoder, outputsDecoder, signatureKey);
        String whisperOutput = _dictionary.tokensToString(new int[1][224]);
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

    private static MappedByteBuffer loadWhisperModel(AssetManager assets, String modelName)
            throws IOException {
        AssetFileDescriptor fileDescriptor = assets.openFd(modelName);
        FileInputStream inputStream = new FileInputStream(fileDescriptor.getFileDescriptor());
        FileChannel fileChannel = inputStream.getChannel();
        long startOffset = fileDescriptor.getStartOffset();
        long declaredLength = fileDescriptor.getDeclaredLength();
        return fileChannel.map(FileChannel.MapMode.READ_ONLY, startOffset, declaredLength);
    }

}