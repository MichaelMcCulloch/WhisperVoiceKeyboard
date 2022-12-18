use crate::{extract, statics::WHISPER_FILTERS};
use ndk::asset::Asset;

pub(crate) fn init(mut buffer: Asset) {
    android_logger::init_once(android_logger::Config::default().with_min_level(log::Level::Trace));

    match unsafe { WHISPER_FILTERS.take() } {
        None => {
            let filter = extract::extract_filters(&mut buffer).unwrap();
            unsafe { WHISPER_FILTERS.replace(filter) };
        }
        _ => {
            log::info!("Library already inititalized");
            unimplemented!()
        }
    }
}
pub(crate) fn uninit() {
    unsafe {
        WHISPER_FILTERS.take();
    }
}
