#!/bin/sh

source .vscode/cargoNdkEnv.sh
cargo ndk -p 33 --bindgen --target=aarch64-linux-android --target=x86_64-linux-android --target=armv7-linux-androideabi --target=i686-linux-android build
cargo ndk -p 33 --bindgen --target=aarch64-linux-android --target=x86_64-linux-android --target=armv7-linux-androideabi --target=i686-linux-android build --release

