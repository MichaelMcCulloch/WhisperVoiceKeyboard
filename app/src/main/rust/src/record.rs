use std::{
    mem::ManuallyDrop,
    sync::mpsc::{channel, Receiver},
    thread,
};

use jni::objects::JByteBuffer;

use crate::{
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
                thread::spawn(move || audio_job(device_id, sample_rate, channels, recv));

            unsafe {
                AUDIO_PROCESSING_THREAD.replace(join_handle);
                AUDIO_PROCESSING_THREAD_MESSENGER.replace(sender);
            }
            true
        }
        (_, _) => {
            log::error!("Cannot start voice thread, already started!");
            false
        }
    }
}

fn audio_job(device_id: i32, sample_rate: i32, channels: i32, recv: Receiver<Message>) -> bool {
    match recv.recv() {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub(crate) fn request_end(output_buffer: ManuallyDrop<Vec<u8>>) -> bool {
    match unsafe {
        (
            AUDIO_PROCESSING_THREAD.take(),
            AUDIO_PROCESSING_THREAD_MESSENGER.take(),
        )
    } {
        (Some(job), Some(sender)) => {
            sender.send(Message::Stop(output_buffer)).unwrap();
            match job.join() {
                Ok(job_success) => job_success,
                Err(_) => {
                    log::error!("Cannot stop voice thread, Failure Joining Thread!");
                    false
                }
            }
        }
        (_, _) => {
            log::error!("Cannot stop voice thread, not started!");
            false
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
                    log::error!("Cannot stop voice thread, Failure Joining Thread!");
                    false
                }
            }
        }
        (_, _) => {
            log::error!("Cannot stop voice thread, not started!");
            false
        }
    }
}
