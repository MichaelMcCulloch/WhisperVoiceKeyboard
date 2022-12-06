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
import java.nio.ByteBuffer;
import java.util.Arrays;
import java.util.Optional;
import java.util.OptionalInt;

public class VoiceKeyboardInputMethodService extends InputMethodService {

    private Optional<RecordingThread> _recordingThread;

    @Override
    public void onCreate() {
        super.onCreate();


        RecordingThread recordingThread = new RecordingThread(getMediaRecorder());
        _recordingThread = Optional.of(recordingThread);
        recordingThread.run();
    }

    @Override
    public void onDestroy() {
        _recordingThread = Optional.empty();
        super.onDestroy();
    }

    @Override
    public View onCreateInputView() {
        View inputView =
                getLayoutInflater().inflate(R.layout.keyboard, null);
        ToggleButton recordButton = inputView.findViewById(R.id.buttonRecord);

        recordButton.setOnCheckedChangeListener((button, checked) -> {
            if (checked && getBottomMicrophone().isPresent()) {
                byte[] bytes = {1, 2, 3, 4};
                byte[] bytes_out = {0, 0, 0, 0};
                ByteBuffer bb = ByteBuffer.allocateDirect(4);
                ByteBuffer bo = ByteBuffer.allocateDirect(4);
                bb.put(bytes);
                bo.put(bytes_out);
                new RustLib().createLogMelSpectrogramFromAudioBytes(bb, bo);

                Log.i("createLogMelSpectrogramFromAudioBytes", "onCreateInputView: " + bo.get(0) + bo.get(1) + bo.get(2) + bo.get(3));
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
        recorder.setOutputFile(new File(getCacheDir(), "whisper_keyboard_user_voice.mp4"));
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
        boolean _success = file.delete();
        _recordingThread.ifPresent(RecordingThread::startRecording);
    }

    private void abortRecording() {
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