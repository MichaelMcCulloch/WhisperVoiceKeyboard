#!/bin/sh

source .vscode/cargoNdkEnv.sh
cargo ndk -p 33 --bindgen  --target=x86_64-linux-android test

