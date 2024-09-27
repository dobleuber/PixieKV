use littlefs2::{consts, driver, io};
use core::result::Result::{Ok, Err};
use core::default::Default;

use crate::domain::constants::MAX_SIZE;

pub struct KVStorage {
    data: [u8; MAX_SIZE], // 1MB of storage
}

impl KVStorage {
    pub fn new() -> Self {
        Self {
            data:  [0xFFu8; MAX_SIZE],
        }
    }
}

impl Default for KVStorage {
    fn default() -> Self {
        Self::new()
    }
}

impl driver::Storage for KVStorage {
    const READ_SIZE: usize = 16;
    const WRITE_SIZE: usize = 16;
    const BLOCK_SIZE: usize = 512;
    const BLOCK_COUNT: usize = 2048; // 1MB / 512B = 2048 blocks
    type CACHE_SIZE = consts::U32; // Increased for better performance
    type LOOKAHEAD_SIZE = consts::U1; // Increased for better performance

    fn read(&mut self, offset: usize, buffer: &mut [u8]) -> io::Result<usize> {
        if offset + buffer.len() > self.data.len() {
            return Err(io::Error::Invalid);
        }
        buffer.copy_from_slice(&self.data[offset..offset + buffer.len()]);
        Ok(buffer.len())
    }

    fn write(&mut self, offset: usize, buffer: &[u8]) -> io::Result<usize> {
        if offset + buffer.len() > self.data.len() {
            return Err(io::Error::Invalid);
        }
        self.data[offset..offset + buffer.len()].copy_from_slice(buffer);
        Ok(buffer.len())
    }

    fn erase(&mut self, offset: usize, len: usize) -> io::Result<usize> {
        if offset + len > self.data.len() {
            return Err(io::Error::Invalid);
        }
        self.data[offset..offset + len].fill(0xFF);
        Ok(len)
    }
}
