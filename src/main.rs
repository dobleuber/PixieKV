#![no_main]
#![no_std]

use panic_halt as _;
use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};

use pixie_kv::domain::pixie_kv_store::PixieKVStore;
use pixie_kv::domain::pixie_kv::PixieKV;
use pixie_kv::domain::persistent::Error;
use littlefs2::fs::{Filesystem, Allocation};
use pixie_kv::domain::storage::KVStorage;
use heapless::String;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Person<const N: usize> {
    name: String<N>,
    age: u8,
    height: u8,
}

impl <const N: usize> Person<N> {
    fn new(name: &str, age: u8, height: u8) -> Self {
        Person { name: String::<N>::try_from(name).unwrap(), age, height }
    }
}

type Person32 = Person<32>;

#[entry]
fn main() -> ! {
    hprintln!("Hello, world!").unwrap();

    // let mut db: PixieKVStore<Person32> = PixieKVStore::default();

    // exit QEMU
    // NOTE do not run this on hardware; it can corrupt OpenOCD state
    debug::exit(debug::EXIT_SUCCESS);

    loop {}
}
