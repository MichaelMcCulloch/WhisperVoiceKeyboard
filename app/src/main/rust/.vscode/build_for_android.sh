#!/bin/sh

source .vscode/cargoNdkEnv.sh
33cargo ndk -p 30 --bindgen --target=aarch64-linux-android   --target=x86_64-linux-android   clean
cargo ndk -p 30 --bindgen --target=aarch64-linux-android   --target=x86_64-linux-android   clean --release
cargo ndk -p 30 --bindgen --target=aarch64-linux-android   --target=x86_64-linux-android   build
cargo ndk -p 30 --bindgen --target=aarch64-linux-android   --target=x86_64-linux-android   build --release
