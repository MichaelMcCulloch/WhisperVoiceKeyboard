# Whisper-based Voice Keyboard

Integration of the OpenAI speech to text model.

## Preparation

### Prerequisites
- [Rust](https://rustup.rs/)
    - [Rust Android Targets](https://github.com/mozilla/rust-android-gradle#usage)
        - `rustup target add armv7-linux-androideabi`   # for arm
        - `rustup target add i686-linux-android`        # for x86
        - `rustup target add aarch64-linux-android`     # for arm64
        - `rustup target add x86_64-linux-android`      # for x86_64
        - `rustup target add x86_64-unknown-linux-gnu`  # for linux-x86-64
        - `rustup target add x86_64-apple-darwin`       # for darwin x86_64 (if you have an Intel MacOS)
        - `rustup target add aarch64-apple-darwin`      # for darwin arm64 (if you have a M1 MacOS)
        - `rustup target add x86_64-pc-windows-gnu`     # for win32-x86-64-gnu
        - `rustup target add x86_64-pc-windows-msvc`    # for win32-x86-64-msvc
    - [Cargo NDK](https://github.com/bbqsrc/cargo-ndk)

### Tools
- [Android Studio](https://developer.android.com/studio)
    - Tools -> SDK Manager -> SDK Tools ->
        - Android SDK Build-Tools 33
        - NDK 25.1.8937393	
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
- Android Studio: Just open the project. Need to expand project view to see rust files. Primarily for editing the Android App source.
- VSCode. Nothing special. This editor is configured to provide `cargo ndk check` on save within this editor. Primarily for editing the rust source.