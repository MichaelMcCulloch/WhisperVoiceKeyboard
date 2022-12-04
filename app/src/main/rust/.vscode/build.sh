#!/bin/sh

source .vscode/cargoNdkEnv.sh


# cargo ndk -p 30 --bindgen --target=aarch64-linux-android   --target=x86_64-linux-android   clean
# cargo ndk -p 30 --bindgen --target=aarch64-linux-android   --target=x86_64-linux-android   clean --release

export FFMPEG_INCLUDE_DIR=/home/michael/AndroidStudioProjects/WhisperVoiceKeyboard/app/src/main/jniInclude/arm64-v8a/
export FFMPEG_LIB_DIR=/home/michael/AndroidStudioProjects/WhisperVoiceKeyboard/app/src/main/jniLibs/arm64-v8a/
cargo ndk -p 33 --bindgen --target=aarch64-linux-android   build
cargo ndk -p 33 --bindgen --target=aarch64-linux-android   build --release

export FFMPEG_INCLUDE_DIR=/home/michael/AndroidStudioProjects/WhisperVoiceKeyboard/app/src/main/jniInclude/x86_64/
export FFMPEG_LIB_DIR=/home/michael/AndroidStudioProjects/WhisperVoiceKeyboard/app/src/main/jniLibs/x86_64/
cargo ndk -p 33 --bindgen --target=x86_64-linux-android     build
cargo ndk -p 33 --bindgen --target=x86_64-linux-android     build --release
# cargo ndk -p 33 --bindgen --target=aarch64-linux-android   --target=x86_64-linux-android   build --release
