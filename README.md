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
        - Android SDK Build-Tools 33.0.0
        - NDK 25.1.8937393
- [VSCode](https://code.visualstudio.com/)
    - Extensions
        - [Rust Analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)


### Assets (Included)
- `whisper.tflite` TFLite Model + `filters_vocab_gen.bin`
    - [How it was made](https://github.com/openai/whisper/discussions/11)
    - [Where I found it](https://github.com/openai/whisper/discussions/506)
    - [Where I got it](https://github.com/openai/whisper/discussions/443)


## How to run
- VSCode:
    - Open the vscode project at `/WhisperVoiceKeyboard/app/src/main/rust` and verify the environment variables set in `.vscode/settings.json::rust-analyzer.server.extraEnv` and `.vscode/cargoNdkEnv.sh`
    - run `./.vscode/buildAll.sh` to build the rust artifacts.
- Android Studio: Just open the project. Need to expand project view to see rust files. Primarily for editing the Android App source.
