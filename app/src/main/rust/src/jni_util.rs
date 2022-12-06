use std::mem::ManuallyDrop;

use jni::{objects::JByteBuffer, JNIEnv};

pub(crate) fn read_jbyte_buffer(
    env: JNIEnv,
    audio_buffer: JByteBuffer,
) -> anyhow::Result<ManuallyDrop<Vec<u8>>> {
    let addr = env.get_direct_buffer_address(audio_buffer)?;
    let capacity = env.get_direct_buffer_capacity(audio_buffer)?;
    let bytes = unsafe { Vec::from_raw_parts(addr, capacity, capacity) };
    Ok(ManuallyDrop::new(bytes))
}
