#[cfg(any(test, debug_assertions))]
pub const MAX_SIZE: usize = 1024;
#[cfg(any(test, debug_assertions))]
pub const MAX_KEY_LEN: usize = 128;
#[cfg(any(test, debug_assertions))]
pub const BLOCK_SIZE: usize = 512;
#[cfg(any(test, debug_assertions))]
pub const BLOCK_COUNT: usize = 2048;


#[cfg(not(any(test, debug_assertions)))]
pub const MAX_SIZE: usize = 128;
#[cfg(not(any(test, debug_assertions)))]
pub const MAX_KEY_LEN: usize = 32;
#[cfg(not(any(test, debug_assertions)))]
pub const BLOCK_SIZE: usize = 128;
#[cfg(not(any(test, debug_assertions)))]
pub const BLOCK_COUNT: usize = 256;
