# Whisper-based Voice Keyboard

Integration of the OpenAI speech to text model.

## Preparation

### Prerequisites
- [Rust](https://rustup.rs/)
    - [Rust Android Targets](https://github.com/mozilla/rust-android-gradle#usage)
        - `rustup target add aarch64-linux-android`     # for arm64
        - `rustup target add x86_64-linux-android`      # for x86_64
    - [Cargo NDK](https://github.com/bbqsrc/cargo-ndk)

### Tools
- [Android Studio](https://developer.android.com/studio)
    - Tools -> SDK Manager -> SDK Tools ->
        - Android SDK Build-Tools 33
        - NDK 221.4.7075529	
- [VSCode](https://code.visualstudio.com/)
    - Extensions
        - [Rust Analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
            - Modify .vscode/settings.json `rust-analyzer.cargo.extraEnv/ANDROID_NDK_HOME` to point to your android NDK installation. Alternatively, if this var is already set, you can remove it.

### Assets (Included)
- `whisper.tflite` TFLite Model + `filters_vocab_gen.bin`
    - [How it was made](https://github.com/openai/whisper/discussions/11)
    - [Where I found it](https://github.com/openai/whisper/discussions/506)
    - [Where I got it](https://github.com/openai/whisper/discussions/443)


## How to run
- VSCode.
    - run `./.vscode/build_for_android.sh`.
- Android Studio: Just open the project. Need to expand project view to see rust files. Primarily for editing the Android App source.
