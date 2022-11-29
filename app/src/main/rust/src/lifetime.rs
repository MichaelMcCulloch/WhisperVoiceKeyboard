use crate::{
    asset_helper::{self, get_asset_manager},
    constants::{FILTERS_VOCAB_GEN_BIN, WHISPER_TFLITE},
    statics::{WHISPER_FILTERS, WHISPER_TFLITE_MODEL, WHISPER_VOCAB},
    whisper::filters::WhisperFilters,
};
use android_logger::Config;
use anyhow::{anyhow, Result};
use jni::{objects::JObject, JNIEnv};
use jni_sys::JavaVM;
use log::Level;
use ndk::asset::Asset;
use ndk_sys::AAsset;
use std::{
    ffi::{c_uint, c_void},
    io::Read,
};
use tflitec::interpreter::{Interpreter, Options};

const SIZE_OF_X32_IN_X8: usize = 4;

pub(crate) fn init(env: JNIEnv, context: JObject, asset_manager: JObject) {
    android_logger::init_once(Config::default().with_min_level(Level::Trace));

    init_ndk_rs(env, context).expect("Could not init NDK-rs context");

    init_whisper_model(env, asset_manager).expect("Could not load Whisper Model!!");
    init_filters_vocab_gen(env, asset_manager).expect("Could not load Filters & Vocab!!");
}

fn init_whisper_model(env: JNIEnv, asset_manager: JObject) -> Result<()> {
    match unsafe { WHISPER_TFLITE_MODEL.take() } {
        None => {
            let asset_manager = get_asset_manager(env, asset_manager);
            let mut tflite_file = asset_helper::load_asset_buffer(WHISPER_TFLITE, &asset_manager)?;
            let tflite_buf = tflite_file.get_buffer()?;

            let options = Options::default();
            let interpreter =
                Interpreter::with_model_bytes(tflite_buf, tflite_buf.len() as u64, Some(options))?;
            unsafe {
                WHISPER_TFLITE_MODEL.replace(interpreter);
            };
            log::info!("Succeeded in Loading Whisper Model!");
            Ok(())
        }
        _ => Err(anyhow!("Model is already populated")),
    }
}

fn init_filters_vocab_gen(env: JNIEnv, asset_manager: JObject) -> Result<()> {
    match unsafe { (WHISPER_VOCAB.take(), WHISPER_FILTERS.take()) } {
        (None, None) => {
            let asset_manager = get_asset_manager(env, asset_manager);
            let mut filters_vocab_gen_bin =
                asset_helper::load_asset_buffer(FILTERS_VOCAB_GEN_BIN, &asset_manager)?;

            if read_u32(&mut filters_vocab_gen_bin)? == 0x5553454e {
                let n_mel = read_i32(&mut filters_vocab_gen_bin)?;
                let n_fft = read_i32(&mut filters_vocab_gen_bin)?;
                assert_eq!(80, n_mel);
                assert_eq!(201, n_fft);

                let data = read_vec_f32(filters_vocab_gen_bin, (n_mel * n_fft) as usize)?;

                unsafe { WHISPER_FILTERS.replace(WhisperFilters::new(n_mel, n_fft, data)) };

                log::info!("Succeeded in Loading Filters & Vocab!");
                Ok(())
            } else {
                Err(anyhow!("Bad Magic"))
            }
        }
        (_, _) => Err(anyhow!("Filters & Vocab are already populated")),
    }
}

fn read_vec_f32(
    mut filters_vocab_gen_bin: Asset,
    number_of_bytes: usize,
) -> Result<Vec<f32>, anyhow::Error> {
    let mut data = vec![0u8; SIZE_OF_X32_IN_X8 * number_of_bytes];
    filters_vocab_gen_bin.read_exact(&mut data)?;
    let data: Vec<f32> = unsafe {
        let (_, floats, _) = data.as_slice().align_to::<f32>();
        floats.to_vec()
    };
    Ok(data)
}

fn read_u32(asset: &mut Asset) -> anyhow::Result<u32> {
    let mut buffer = [0u8; SIZE_OF_X32_IN_X8];
    asset.read_exact(&mut buffer)?;
    Ok(u32::from_le_bytes(buffer))
}

fn read_i32(asset: &mut Asset) -> anyhow::Result<i32> {
    let mut buffer = [0u8; SIZE_OF_X32_IN_X8];
    asset.read_exact(&mut buffer)?;
    Ok(i32::from_le_bytes(buffer))
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
