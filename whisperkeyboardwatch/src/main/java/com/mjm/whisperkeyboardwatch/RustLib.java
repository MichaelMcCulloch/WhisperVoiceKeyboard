package com.mjm.whisperkeyboardwatch;

import android.content.res.AssetManager;

import java.nio.ByteBuffer;
import java.nio.ByteOrder;
import java.nio.FloatBuffer;
import java.util.Optional;

public class RustLib {

    static {
        System.loadLibrary("rust");
    }


    private static boolean isRecording = false;

    /**
     * Starts a recording session using the provided AudioDeviceConfig.
     *
     * @param deviceConfig the AudioDeviceConfig to use for this recording session.
     * @return boolean - true if recording was successfully started, false otherwise.
     */
    public static boolean startRecording(AudioDeviceConfig deviceConfig) {
        if (isRecording) {
            return false;
        }
        isRecording = true;
        return startRecording(deviceConfig.getDeviceId(), deviceConfig.getDeviceSampleRate(), deviceConfig.getDeviceChannels());
    }

    /**
     * Ends the recording session and returns the spectrogram acquired since startRecording was triggered.
     *
     * @return Optional.of(float[]): an array of floats representing the spectrogram of the recorded audio signal. If recording was not active, returns Optional.empty().
     */
    public static Optional<float[]> endRec() {

        if (!isRecording) {
            return Optional.empty();
        }
        isRecording = false;

        ByteBuffer buffer = endRecording();
        if (buffer.capacity() != 0) {
            buffer.order(ByteOrder.LITTLE_ENDIAN);
            FloatBuffer floatBuffer = buffer.asFloatBuffer().asReadOnlyBuffer();
            floatBuffer.rewind();
            float[] ff = new float[240000];
            FloatBuffer _f = floatBuffer.get(ff);
            return Optional.of(ff);
        }
        return Optional.empty();
    }

    public static native void init(AssetManager assetManager);

    public static native void uninit();

    private static native boolean startRecording(int deviceId, int sampleRate, int channels);

    public static boolean abortRecording() {
        if (!isRecording) {
            return false;
        }
        isRecording = false;
        return abortRec();
    }


    private static native boolean abortRec();

    private static native ByteBuffer endRecording();


}