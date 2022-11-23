#!/bin/sh


export ANDROID_SDK_HOME="/home/michael/Android/Sdk"
export ANDROID_NDK_API_LEVEL=21
export ANDROID_SDK_VERSION=21
export ANDROID_API_LEVEL=21
export ANDROID_BUILD_TOOLS_VERSION=33.0.0
export ANDROID_NDK_HOME="$ANDROID_SDK_HOME/ndk/21.4.7075529"
export ANDROID_HOME="$ANDROID_SDK_HOME/build-tools/21.1.2"
export BINDGEN_EXTRA_CLANG_ARGS_x86_64_LINUX_ANDROID="-I/home/michael/Android/Sdk/ndk/21.4.7075529/sources/cxx-stl/llvm-libc++/include/ -I/home/michael/Android/Sdk/ndk/21.4.7075529/sysroot/usr/include/ -I/home/michael/Android/Sdk/ndk/21.4.7075529/toolchains/llvm/prebuilt/linux-x86_64/sysroot/usr/include/x86_64_linux_android/",
export BINDGEN_EXTRA_CLANG_ARGS_AARCH64_LINUX_ANDROID="-I/home/michael/Android/Sdk/ndk/21.4.7075529/sources/cxx-stl/llvm-libc++/include/ -I/home/michael/Android/Sdk/ndk/21.4.7075529/sysroot/usr/include/ -I/home/michael/Android/Sdk/ndk/21.4.7075529/toolchains/llvm/prebuilt/linux-x86_64/sysroot/usr/include/aarch64_linux_android/"
