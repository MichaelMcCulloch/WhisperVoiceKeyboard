#!/bin/sh

source .vscode/cargoNdkEnv.sh
cargo ndk -p 33 --bindgen --target=aarch64-linux-android  fix --allow-dirty --allow-staged


