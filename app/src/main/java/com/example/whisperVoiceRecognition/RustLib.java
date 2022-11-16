package com.example.whisperVoiceRecognition;

public class RustLib  {
    static {
        System.loadLibrary("rust");
    }
    public String helloWorld() {
       return hello("World");
    }

    private static native String hello(String input);

}
