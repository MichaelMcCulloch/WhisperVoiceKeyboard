package com.example.whisperVoiceRecognition;

import android.content.res.AssetManager;

public class RustLib  {
    static {
        System.loadLibrary("rust");
    }
    public String helloWorld() {
       return hello("World");
    }
    public String retrieveAssetPub(AssetManager assetManager) {
       return retrieveAsset(assetManager);
    }

    private static native String hello(String input);
    private static native String retrieveAsset(AssetManager assetManager);

}
