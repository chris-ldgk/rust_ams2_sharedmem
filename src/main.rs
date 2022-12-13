use std::{mem::size_of, ffi::c_char};
mod SMS_MemMap;
use SMS_MemMap::SharedMemory;
use shared_memory::ShmemConf;

use crate::SMS_MemMap::from_slice_u8_to_SharedMemory;

fn zascii(slice: &[c_char]) -> String {
    String::from_iter(slice.iter().take_while(|c| **c != 0).map(|c| *c as u8 as char))
  }

fn main() {
    let shmem = match ShmemConf::new().os_id("$pcars2$").size(size_of::<SharedMemory>()).open() {
        Ok(m) => m,
        Err(e) =>  {
            eprintln!("Unable to create or open shmem: {}", e);
            return;
        }
    };

    unsafe {
        println!("Is owner: {}", shmem.is_owner());
        loop {
            let slice = shmem.as_slice();
            let shared_data: &SharedMemory = from_slice_u8_to_SharedMemory(slice);
            println!("Game version: {}, Player ID: {:?}", shared_data.mVersion, zascii(&shared_data.mParticipantInfo[0].mName as &[i8]));
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }
}