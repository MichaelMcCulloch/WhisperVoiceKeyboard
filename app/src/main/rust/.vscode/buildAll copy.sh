#!/bin/bash

source .vscode/cargoNdkEnv.sh
# Set path to ndk-bundle
export NDK_BUNDLE_DIR=$ANDROID_NDK_HOME

# Export PATH to contain directories of clang and aarch64-linux-android-* utilities
export PATH=${NDK_BUNDLE_DIR}/toolchains/aarch64-linux-android-$ANDROID_NDK_API_LEVEL/prebuilt/linux-x86_64/bin/:${NDK_BUNDLE_DIR}/toolchains/llvm/prebuilt/linux-x86_64/bin:$PATH

# Setup LDFLAGS so that loader can find libgcc and pass -lm for sqrt
export LDFLAGS="-L${NDK_BUNDLE_DIR}/toolchains/aarch64-linux-android-$ANDROID_NDK_API_LEVEL/prebuilt/linux-x86_64/lib/gcc/aarch64-linux-android/$ANDROID_NDK_API_LEVEL.x -lm"

# Download openblas source
git clone https://github.com/xianyi/OpenBLAS.git

# Setup the clang cross compile options
export CLANG_FLAGS="-target aarch64-linux-android --sysroot ${NDK_BUNDLE_DIR}/platforms/android-$ANDROID_API_LEVEL/arch-arm64 -gcc-toolchain ${NDK_BUNDLE_DIR}/toolchains/aarch64-linux-android-$ANDROID_NDK_API_LEVEL/prebuilt/linux-x86_64/"

# Compile
cd OpenBLAS
make TARGET=ARMV8 ONLY_CBLAS=1 AR=ar CC="clang ${CLANG_FLAGS}" HOSTCC=gcc -j4

# Copy libopenblas.so to JNI_LIB_DIR
cp OpenBLAS/libopenblas.so $JNI_LIB_DIR

# Link libopenblas.so in rust project
cargo ndk -p 33 --bindgen --target=aarch64-linux-android --target=x86_64-linux-android build
cargo ndk -p 33 --bindgen --target=aarch64-linux-android --target=x86_64-linux-android build --release