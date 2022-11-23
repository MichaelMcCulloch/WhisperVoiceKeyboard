#!/bin/sh

source .vscode/cargoNdkEnv.sh
# cargo ndk --bindgen --target=aarch64-linux-android   --target=x86_64-linux-android   clean
# cargo ndk --bindgen --target=aarch64-linux-android   --target=x86_64-linux-android   clean --release
cargo ndk -p 33 --bindgen --target=aarch64-linux-android   --target=x86_64-linux-android   build
# cargo ndk --bindgen --target=aarch64-linux-android   --target=x86_64-linux-android   build --release
