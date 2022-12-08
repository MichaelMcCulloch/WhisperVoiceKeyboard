package com.mjm.whisperVoiceRecognition;

import android.content.res.AssetManager;

import java.nio.ByteBuffer;
import java.util.Optional;

public class RustLib {

    static {
        System.loadLibrary("rust");
    }


    public static boolean startRecording(AudioDeviceConfig deviceConfig) {
        return startRecording(deviceConfig.getDeviceId(), deviceConfig.getDeviceSampleRate(), deviceConfig.getDeviceChannels());
    }

    public static Optional<ByteBuffer> endRec() {
        ByteBuffer buffer = endRecording();
        if (buffer.capacity() != 0) {
            return Optional.of(buffer);
        }
        return Optional.empty();
    }

    public static native void init(AssetManager assetManager);

    public static native void uninit();

    private static native boolean startRecording(int deviceId, int sampleRate, int channels);

    private static native ByteBuffer endRecording();

    private static native boolean abortRecording();


}
