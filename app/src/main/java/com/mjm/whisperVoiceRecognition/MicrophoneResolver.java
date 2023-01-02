package com.mjm.whisperVoiceRecognition;

import android.media.AudioDeviceInfo;
import android.media.AudioManager;

import java.util.Arrays;
import java.util.Optional;
import java.util.OptionalInt;
import java.util.function.Predicate;
import java.util.logging.Logger;

class MicrophoneResolver {
    private static final Logger logger = Logger.getLogger(MicrophoneResolver.class.getName());

    private final AudioManager audioManager;

    MicrophoneResolver(AudioManager audioManager) {
        this.audioManager = audioManager;
    }

    Optional<AudioDeviceConfig> resolveMicrophone() {
        Optional<AudioDeviceConfig> bottomMicrophone = getMicrophone(audioDeviceInfo -> audioDeviceInfo.getAddress().equals("bottom"));

        if (bottomMicrophone.isPresent()) {
            return bottomMicrophone;
        }

        return getMicrophone(audioDeviceInfo -> !audioDeviceInfo.getAddress().trim().isEmpty());
    }

    private Optional<AudioDeviceConfig> getMicrophone(final Predicate<AudioDeviceInfo> audioDeviceInfoFilter) {
        AudioDeviceInfo[] adi = audioManager.getDevices(AudioManager.GET_DEVICES_INPUTS);
        Optional<AudioDeviceInfo> audioDeviceInfoMatchingFilter = Arrays.stream(adi)
                .filter(audioDeviceInfoFilter)
                .findFirst();

        if (audioDeviceInfoMatchingFilter.isPresent()) {
            logger.fine("Found microphone: " + audioDeviceInfoMatchingFilter.get().getAddress());
            return getMicrophoneFromAudioDeviceInfo(audioDeviceInfoMatchingFilter.get());
        }
        return Optional.empty();
    }

    private Optional<AudioDeviceConfig> getMicrophoneFromAudioDeviceInfo(AudioDeviceInfo audioDeviceInfo) {
        OptionalInt maxSampleRate = Arrays.stream(audioDeviceInfo.getSampleRates())
                .max();
        OptionalInt minChannels = Arrays.stream(audioDeviceInfo.getChannelCounts())
                .min();
        if (maxSampleRate.isPresent() && minChannels.isPresent()) {
            AudioDeviceConfig audioDeviceConfig = new AudioDeviceConfig(audioDeviceInfo.getId(), maxSampleRate.getAsInt(), minChannels.getAsInt());

            return Optional.of(audioDeviceConfig);
        }
        return Optional.empty();
    }
}
