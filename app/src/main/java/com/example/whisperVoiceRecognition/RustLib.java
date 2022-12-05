package com.example.whisperVoiceRecognition;

import java.nio.ByteBuffer;

public class RustLib {

    static {
//        System.loadLibrary("avutil");
//        System.loadLibrary("avcodec");
//        System.loadLibrary("avformat");
//        System.loadLibrary("swresample");
//        System.loadLibrary("swscale");
//        System.loadLibrary("avfilter");
//        System.loadLibrary("avdevice");
        System.loadLibrary("rust");

        RustLib.initLogger();
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
    public static native void initLogger();

    public native boolean createLogMelSpectogramFromAudioBytes(ByteBuffer audio, ByteBuffer output);


}
