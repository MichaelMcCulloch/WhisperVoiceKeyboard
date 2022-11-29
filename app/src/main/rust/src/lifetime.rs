use std::ffi::c_void;

use android_logger::Config;
use anyhow::{anyhow, Result};
use jni::{objects::JObject, JNIEnv};
use jni_sys::JavaVM;
use log::Level;
use tflitec::interpreter::{Interpreter, Options};

use crate::{
    asset_helper::{self, get_asset_manager},
    constants::WHISPER_TFLITE,
    statics::WHISPER_TFLITE_MODEL,
};

pub(crate) fn init(env: JNIEnv, context: JObject, asset_manager: JObject) {
    android_logger::init_once(Config::default().with_min_level(Level::Trace));

    init_ndk_rs(env, context).expect("Could not init NDK-rs context");

    init_whisper_model(env, asset_manager).expect("Could not load Whisper Model!!");
}

fn init_whisper_model(env: JNIEnv, asset_manager: JObject) -> Result<()> {
    match unsafe { WHISPER_TFLITE_MODEL.take() } {
        Some(_) => Err(anyhow!("Model is already populated")),
        None => {
            let asset_manager = get_asset_manager(env, asset_manager);
            let mut tflite_file = asset_helper::load_asset_buffer(WHISPER_TFLITE, &asset_manager)?;
            let tflite_buf = tflite_file.get_buffer()?;
            let interpreter = Interpreter::with_model_bytes(
                tflite_buf,
                tflite_buf.len() as u64,
                Some(Options::default()),
            )?;
            unsafe {
                WHISPER_TFLITE_MODEL.replace(interpreter);
            };
            log::info!("Succeeded in Loading Whisper Model!");
            Ok(())
        }
    }
}

fn init_ndk_rs(env: JNIEnv, mut context: JObject) -> Result<()> {
    unsafe {
        let vm: *mut JavaVM = env.get_java_vm()?.get_java_vm_pointer();
        ndk_context::initialize_android_context(
            vm as *mut _ as *mut c_void,
            &mut context as *mut _ as *mut c_void,
        );
    }
    log::info!("Succeeded in init NDK-rs context");
    Ok(())
}

pub(crate) fn uninit() {
    unsafe {
        ndk_context::release_android_context();
    }
    log::info!("Succeeded in deinit NDK-rs context");

    unsafe {
        let _model = WHISPER_TFLITE_MODEL.take();
    }
    log::info!("Succeeded in unloading Whisper Model");
}
