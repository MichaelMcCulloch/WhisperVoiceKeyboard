package com.mjm.whisperkeyboardwatch;

public class AudioDeviceConfig {
    private final int deviceId;
    private final int deviceSampleRate;
    private final int deviceChannels;


    public AudioDeviceConfig(int deviceId, int deviceSampleRate, int deviceChannels) {
        this.deviceId = deviceId;
        this.deviceSampleRate = deviceSampleRate;
        this.deviceChannels = deviceChannels;
    }

    public int getDeviceId() {
        return deviceId;
    }

    public int getDeviceSampleRate() {
        return deviceSampleRate;
    }

    public int getDeviceChannels() {
        return deviceChannels;
    }
}
