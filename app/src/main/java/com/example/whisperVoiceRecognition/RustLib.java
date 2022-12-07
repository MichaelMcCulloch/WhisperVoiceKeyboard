package com.example.whisperVoiceRecognition;

import android.util.Log;

import java.nio.ByteBuffer;
import java.util.Optional;

public class RustLib {

    static {

        System.loadLibrary("rust");

    }

    //
//    public void retrieveAssetPub(AssetManager assetManager) {
//        retrieveAsset(assetManager);
//    }
//
//    private static native String hello(String input);
//    private static native void retrieveAsset(AssetManager assetManager);
//    public static native void sampleAudio();
//
//    /**
//     * @param context ApplicationContext
//     * @param deviceId AudioManager Device id for the microphone
//     * @param sampleRate AudioManager sample rate for the device
//     * @param channels AudioManager Channels for the device
//     */
//    public static native void initLogger();
//
//    public native boolean createLogMelSpectrogramFromAudioBytes(ByteBuffer audio, ByteBuffer output);

    public static boolean startRecording(AudioDeviceConfig deviceConfig) {
        return RustLib.startRecording(deviceConfig.getDeviceId(), deviceConfig.getDeviceSampleRate(), deviceConfig.getDeviceChannels());
    }

    public static Optional<ByteBuffer> endRec() {
        ByteBuffer buffer = RustLib.endRecording();
        if (buffer.capacity() != 0) {
            Log.i("BUFFER", "endRec: " + buffer.capacity());
            return Optional.of(buffer);
        }
        return Optional.empty();

    }

    public static native void init();

    public static native void uninit();

    private static native boolean startRecording(int deviceId, int sampleRate, int channels);

    private static native ByteBuffer endRecording();

    private static native boolean abortRecording();


}
