package com.example.whisperVoiceRecognition;

import android.content.Context;
import android.content.res.AssetManager;

public class RustLib  {


    public void retrieveAssetPub(AssetManager assetManager) {
        retrieveAsset(assetManager);
    }

    private static native String hello(String input);
    private static native void retrieveAsset(AssetManager assetManager);
    public static native void init(Context context);

}
