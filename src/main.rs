use std::{ffi::c_char, mem::size_of};
mod SMS_MemMap;
use raw_sync::{events::*, Timeout};
use shared_memory::ShmemConf;
use std::thread::sleep;
use std::time::Duration;
use SMS_MemMap::SharedMemory;

use crate::SMS_MemMap::{from_slice_u8_to_SharedMemory, SHARED_MEMORY_VERSION};

fn zascii(slice: &[c_char]) -> String {
    String::from_iter(
        slice
            .iter()
            .take_while(|c| **c != 0)
            .map(|c| *c as u8 as char),
    )
}

fn main() {
    let shmem = match ShmemConf::new()
        .os_id("$pcars2$")
        .size(size_of::<SharedMemory>())
        .open()
    {
        Ok(m) => m,
        Err(e) => {
            eprintln!(
                "Unable to create or open shmem: {}. Is AMS2 running and Shared Memory enabled?",
                e
            );
            return;
        }
    };

    // Loop version
    loop {
        let slice = unsafe { shmem.as_slice() };
        let shared_data: &SharedMemory = from_slice_u8_to_SharedMemory(slice);
        println!(
            "Client version: {}\n
            Game version: {}\n
            Player ID: {}",
            SHARED_MEMORY_VERSION,
            shared_data.mVersion,
            zascii(&shared_data.mParticipantInfo[0].mName as &[i8])
        );
        sleep(Duration::from_secs(1));
    }
}
