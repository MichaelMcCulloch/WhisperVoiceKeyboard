use crate::{
    extract,
    statics::{WHISPER_FILTERS, WHISPER_VOCAB},
};
use ndk::asset::Asset;

pub(crate) fn init(mut buffer: Asset) {
    android_logger::init_once(android_logger::Config::default().with_min_level(log::Level::Trace));

    match unsafe { (WHISPER_VOCAB.take(), WHISPER_FILTERS.take()) } {
        (None, None) => {
            let (filter, vocab) = extract::extract_filters_and_vocab(&mut buffer).unwrap();
            unsafe {
                WHISPER_VOCAB.replace(vocab);
                WHISPER_FILTERS.replace(filter)
            };
        }
        (_, _) => {
            log::info!("Library already inititalized");
            unimplemented!()
        }
    }
}
pub(crate) fn uninit() {
    unsafe {
        WHISPER_VOCAB.take();
        WHISPER_FILTERS.take();
    }
}
