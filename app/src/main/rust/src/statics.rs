use std::{sync::mpsc::Sender, thread::JoinHandle};

use tflitec::interpreter::Interpreter;

use crate::{
    transcription::recording::Message,
    whisper::{filters::WhisperFilters, vocab::WhisperVocab},
};

pub(crate) static mut VOICE_PROCESSING_THREAD: Option<JoinHandle<Option<String>>> = None;
pub(crate) static mut VOICE_PROCESSING_THREAD_MESSENGER: Option<Sender<Message>> = None;
pub(crate) static mut WHISPER_TFLITE_MODEL: Option<Interpreter> = None;
pub(crate) static mut WHISPER_MEL_FILTERS: Option<WhisperFilters> = None;
pub(crate) static mut WHISPER_VOCAB: Option<WhisperVocab> = None;
