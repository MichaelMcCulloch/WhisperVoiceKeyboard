pub(crate) static mut DEVICE_ID: Option<i32> = None;
pub(crate) static mut SAMPLE_RATE: Option<i32> = None;
pub(crate) static mut CHANNELS: Option<i32> = None;

pub(crate) fn init_audio(device_id: i32, sample_rate: i32, channels: i32) {
    let device_id = unsafe { DEVICE_ID.replace(device_id) };
    assert!(device_id.is_none());
    let sample_rate = unsafe { SAMPLE_RATE.replace(sample_rate) };
    assert!(sample_rate.is_none());
    let channels = unsafe { CHANNELS.replace(channels) };
    assert!(channels.is_none());
}

pub(crate) fn uninit_audio() {
    let device_id = unsafe { DEVICE_ID.take() };
    assert!(device_id.is_some());
    let sample_rate = unsafe { SAMPLE_RATE.take() };
    assert!(sample_rate.is_some());
    let channels = unsafe { CHANNELS.take() };
    assert!(channels.is_some());
}
