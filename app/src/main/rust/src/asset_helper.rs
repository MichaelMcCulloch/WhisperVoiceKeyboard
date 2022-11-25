use std::{ffi::CString, io::Error, ptr::NonNull};

use jni::{objects::JObject, JNIEnv};
use ndk::asset::{Asset, AssetManager};

pub fn load_asset_buffer(asset_name: &str, asset_manager: &AssetManager) -> anyhow::Result<Asset> {
    let asset_cstring = CString::new(asset_name)?;
    let asset = asset_manager
        .open(&asset_cstring)
        .ok_or(anyhow::Error::new(Error::new(
            std::io::ErrorKind::NotFound,
            format!("Error opening {:?}", asset_name),
        )))?;

    Ok(asset)
}

pub fn get_asset_manager(env: JNIEnv, asset_manager_object: JObject) -> AssetManager {
    let aasset_manager_pointer = unsafe {
        ndk_sys::AAssetManager_fromJava(env.get_native_interface(), *asset_manager_object)
    };
    let asset_manager = unsafe {
        ndk::asset::AssetManager::from_ptr(NonNull::<ndk_sys::AAssetManager>::new_unchecked(
            aasset_manager_pointer,
        ))
    };
    asset_manager
}
