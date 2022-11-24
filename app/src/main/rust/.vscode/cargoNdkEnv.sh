#!/bin/sh


export ANDROID_SDK_HOME="/home/michael/Android/Sdk"
export ANDROID_NDK_API_LEVEL=30
export ANDROID_SDK_VERSION=30
export ANDROID_API_LEVEL=30
export ANDROID_BUILD_TOOLS_VERSION=33.0.0
export ANDROID_NDK_HOME="$ANDROID_SDK_HOME/ndk/25.1.8937393"
export ANDROID_HOME="$ANDROID_SDK_HOME/build-tools/33.0.0"
export BINDGEN_EXTRA_CLANG_ARGS_X86_64_LINUX_ANDROID="-I/home/michael/Android/Sdk/ndk/25.1.8937393/sources/cxx-stl/llvm-libc++/include/ -I/home/michael/Android/Sdk/ndk/25.1.8937393/sysroot/usr/include/ -I/home/michael/Android/Sdk/ndk/25.1.8937393/toolchains/llvm/prebuilt/linux-x86_64/sysroot/usr/include/x86_64_linux_android/",
export BINDGEN_EXTRA_CLANG_ARGS_AARCH64_LINUX_ANDROID="-I/home/michael/Android/Sdk/ndk/25.1.8937393/sources/cxx-stl/llvm-libc++/include/ -I/home/michael/Android/Sdk/ndk/25.1.8937393/sysroot/usr/include/ -I/home/michael/Android/Sdk/ndk/25.1.8937393/toolchains/llvm/prebuilt/linux-x86_64/sysroot/usr/include/aarch64_linux_android/"
export TFLITEC_PREBUILT_PATH_X86_64_LINUX_ANDROID="/home/michael/AndroidStudioProjects/WhisperVoiceKeyboard/app/src/main/jniLibs/x86_64/libtensorflowlite_c.so"
export TFLITEC_PREBUILT_PATH_AARCH64_LINUX_ANDROID="/home/michael/AndroidStudioProjects/WhisperVoiceKeyboard/app/src/main/jniLibs/arm64-v8a/libtensorflowlite_c.so"
