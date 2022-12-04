#!/bin/sh

export PROJECT_ROOT="$HOME/AndroidStudioProjects/WhisperVoiceKeyboard"
export JNI_LIB_DIR="$PROJECT_ROOT/app/src/main/jniLibs"
export TFLITEC_PREBUILT_PATH_X86_64_LINUX_ANDROID="$JNI_LIB_DIR/x86_64/libtensorflowlite_c.so"
export TFLITEC_PREBUILT_PATH_AARCH64_LINUX_ANDROID="$JNI_LIB_DIR/arm64-v8a/libtensorflowlite_c.so"
export ANDROID_NDK_API_LEVEL=30
export ANDROID_SDK_VERSION=30
export ANDROID_API_LEVEL=30
export ANDROID_BUILD_TOOLS_VERSION=33.0.1
export ANDROID_NDK_VERSION="25.1.8937393"
export ANDROID_SDK_HOME="/home/michael/Android/Sdk"
export ANDROID_HOME="$ANDROID_SDK_HOME/build-tools/$ANDROID_BUILD_TOOLS_VERSION"
export ANDROID_NDK_HOME="$ANDROID_SDK_HOME/ndk/$ANDROID_NDK_VERSION"
export BINDGEN_EXTRA_CLANG_ARGS_X86_64_LINUX_ANDROID="-I$ANDROID_NDK_HOME/sources/cxx-stl/llvm-libc++/include/ -I$ANDROID_NDK_HOME/sysroot/usr/include/ -I$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/sysroot/usr/include/x86_64_linux_android/",
export BINDGEN_EXTRA_CLANG_ARGS_AARCH64_LINUX_ANDROID="-I$ANDROID_NDK_HOME/sources/cxx-stl/llvm-libc++/include/ -I$ANDROID_NDK_HOME/sysroot/usr/include/ -I$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/sysroot/usr/include/aarch64_linux_android/"


export PKG_CONFIG_SYSROOT_DIR="$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/sysroot/"