package com.mjm.whisperVoiceRecognition;

import android.content.res.AssetManager;
import android.util.Log;
import android.util.Pair;

import java.nio.ByteBuffer;
import java.nio.ByteOrder;
import java.nio.FloatBuffer;
import java.util.Optional;

public class RustLib {

    static {
        System.loadLibrary("rust");
    }


    private static boolean isRecording = false;

    public static boolean startRecording(AudioDeviceConfig deviceConfig) {
        if (isRecording) {
            return false;
        }
        isRecording = true;
        return startRecording(deviceConfig.getDeviceId(), deviceConfig.getDeviceSampleRate(), deviceConfig.getDeviceChannels());
    }

    public static Pair<Optional<float[]>, Long> endRec() {

        if (!isRecording) {
            return new Pair<>(Optional.empty(), 0L);
        }
        isRecording = false;

        long alpha = System.nanoTime();
        ByteBuffer buffer = endRecording();
        long beta = System.nanoTime();
        Log.i("RustLib", "endRec: took" + (beta - alpha) + "nanos");
        if (buffer.capacity() != 0) {
            buffer.order(ByteOrder.LITTLE_ENDIAN);
            FloatBuffer floatBuffer = buffer.asFloatBuffer().asReadOnlyBuffer();
            floatBuffer.rewind();
            float[] ff = new float[240000];
            FloatBuffer _f = floatBuffer.get(ff);
            return new Pair<>(Optional.of(ff), beta - alpha);
        }
        return new Pair<>(Optional.empty(), 0L);
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