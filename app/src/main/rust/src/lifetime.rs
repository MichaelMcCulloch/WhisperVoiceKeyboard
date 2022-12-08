use crate::{statics::WHISPER_VOCAB, whisper::vocab::Vocab};

pub(crate) fn init() {
    android_logger::init_once(android_logger::Config::default().with_min_level(log::Level::Trace));
    unsafe {
        WHISPER_VOCAB.replace(Vocab::default());
        WHISPER_VOCAB.replace(Vocab::default());
    }
}
pub(crate) fn uninit() {
    unsafe {
        WHISPER_VOCAB.take();
    }
}
