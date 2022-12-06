use std::{sync::mpsc::Sender, thread::JoinHandle};

use crate::Message;

pub(crate) static mut AUDIO_PROCESSING_THREAD: Option<JoinHandle<bool>> = None;
pub(crate) static mut AUDIO_PROCESSING_THREAD_MESSENGER: Option<Sender<Message>> = None;
