#!/bin/sh


ANDROID_SDK_HOME="/home/michael/Android/Sdk" \
ANDROID_NDK_API_LEVEL=21 \
ANDROID_SDK_VERSION=21 \
ANDROID_API_LEVEL=21 \
ANDROID_BUILD_TOOLS_VERSION=33.0.0 \
ANDROID_NDK_HOME="$ANDROID_SDK_HOME/ndk/21.4.7075529" \
ANDROID_HOME="$ANDROID_SDK_HOME/build-tools/21.1.2" \
cargo ndk --target=aarch64-linux-android   --target=x86_64-linux-android   build -vv
# HOST_TAG=linux-x86_64 \
# TARGET_TRIPLE=aarch64-linux-android \
# BINDGEN_EXTRA_CLANG_ARGS="\
# -I${ANDROID_NDK_HOME}/sources/cxx-stl/llvm-libc++/include/ \
# -I${ANDROID_NDK_HOME}/sysroot/usr/include/ \
# -I${ANDROID_NDK_HOME}/toolchains/llvm/prebuilt/${HOST_TAG}/sysroot/usr/include/${TARGET_TRIPLE}/" \