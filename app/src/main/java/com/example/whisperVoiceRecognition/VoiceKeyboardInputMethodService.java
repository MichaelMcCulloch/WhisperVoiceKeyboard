package com.example.whisperVoiceRecognition;

import android.content.Context;
import android.inputmethodservice.InputMethodService;
import android.media.AudioDeviceInfo;
import android.media.AudioManager;
import android.media.MediaRecorder;
import android.util.Log;
import android.view.View;
import android.widget.ToggleButton;

import androidx.annotation.NonNull;

import com.example.WhisperVoiceKeyboard.R;

import java.io.File;
import java.util.Arrays;
import java.util.Optional;
import java.util.OptionalInt;

public class VoiceKeyboardInputMethodService extends InputMethodService {

    private Optional<RecordingThread> _recordingThread;

    @Override
    public void onCreate() {
        super.onCreate();
        System.loadLibrary("rust");
        RustLib.init(getApplicationContext());

        RecordingThread recordingThread = new RecordingThread(getMediaRecorder());
        _recordingThread = Optional.of(recordingThread);
        recordingThread.run();
    }

    @Override
    public void onDestroy() {
        _recordingThread = Optional.empty();
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
                startRecording();
            } else {
                String result = endRecording();
                getCurrentInputConnection().commitText(result, result.length());
            }
        });

        return inputView;
    }

    @NonNull
    private MediaRecorder getMediaRecorder() {
        MediaRecorder recorder = new MediaRecorder();
        recorder.setAudioSource(MediaRecorder.AudioSource.MIC);
        recorder.setOutputFormat(MediaRecorder.OutputFormat.MPEG_4);
        recorder.setAudioEncoder(MediaRecorder.AudioEncoder.AAC);
        recorder.setAudioChannels(1);
        recorder.setAudioSamplingRate(16000);
        recorder.setOutputFile(new File(getCacheDir(), "audio.mp4"));
        return recorder;
    }


    private String endRecording() {
        _recordingThread.ifPresent(RecordingThread::endRecording);

        File file = new File(getCacheDir(), "audio.mp4");
        Log.i("VoiceKeyboardInputMethodService", "endRecording: " + file.getPath());
        return "Ok";
    }

    private void startRecording() {
        File file = new File(getCacheDir(), "audio.mp4");
        file.delete();
        _recordingThread.ifPresent(RecordingThread::startRecording);
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