use std::{sync::mpsc::channel, thread};

use crate::{
    job,
    statics::{AUDIO_PROCESSING_THREAD, AUDIO_PROCESSING_THREAD_MESSENGER},
    Message,
};

pub(crate) fn request_start(device_id: i32, sample_rate: i32, channels: i32) -> bool {
    match unsafe {
        (
            AUDIO_PROCESSING_THREAD.take(),
            AUDIO_PROCESSING_THREAD_MESSENGER.take(),
        )
    } {
        (None, None) => {
            let (sender, recv) = channel();
            let join_handle =
                thread::spawn(move || job::audio_job(device_id, sample_rate, channels, recv));

            unsafe {
                AUDIO_PROCESSING_THREAD.replace(join_handle);
                AUDIO_PROCESSING_THREAD_MESSENGER.replace(sender);
            }
            log::info!("Starting Voice Thread");
            true
        }
        (_, _) => {
            log::error!("Cannot start voice thread, already started!");
            false
        }
    }
}

pub(crate) fn request_end() -> Option<Vec<u8>> {
    match unsafe {
        (
            AUDIO_PROCESSING_THREAD.take(),
            AUDIO_PROCESSING_THREAD_MESSENGER.take(),
        )
    } {
        (Some(job), Some(sender)) => {
            sender.send(Message::Stop).unwrap();
            let ret = match job.join() {
                Ok(job_success) => job_success,
                Err(_) => {
                    log::error!("Cannot stop voice thread, Failure Joining Thread!");
                    None
                }
            };
            log::info!("Stopping Voice Thread");

            ret
        }
        (_, _) => {
            log::error!("Cannot stop voice thread, not started!");
            None
        }
    }
}

pub(crate) fn request_abort() -> bool {
    match unsafe {
        (
            AUDIO_PROCESSING_THREAD.take(),
            AUDIO_PROCESSING_THREAD_MESSENGER.take(),
        )
    } {
        (Some(job), Some(sender)) => {
            sender.send(Message::Abort).unwrap();
            match job.join() {
                Ok(_) => true,
                Err(_) => {
                    log::error!("Cannot Abort voice thread, Failure Joining Thread!");
                    false
                }
            }
        }
        (_, _) => {
            log::error!("Cannot Abort voice thread, not started!");
            false
        }
    }
}
