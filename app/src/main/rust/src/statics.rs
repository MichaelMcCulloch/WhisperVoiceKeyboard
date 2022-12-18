use std::{sync::mpsc::Sender, thread::JoinHandle};

use crate::Message;

pub(crate) static mut AUDIO_PROCESSING_THREAD: Option<JoinHandle<Option<Vec<u8>>>> = None;
pub(crate) static mut AUDIO_PROCESSING_THREAD_MESSENGER: Option<Sender<Message>> = None;
pub(crate) static mut WHISPER_FILTERS: Option<Vec<Vec<f32>>> = None;
