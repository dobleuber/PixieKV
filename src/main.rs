#![no_main]
#![no_std]

use panic_halt as _;
use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};

use pixie_kv::domain::pixie_kv_store::PixieKVStore;
use pixie_kv::domain::pixie_kv::PixieKV;
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

    // Create a new PixieKVStore
    let mut db: PixieKVStore<Person32> = PixieKVStore::default();

    // Insert some key-value pairs
    db.insert("anne", Person::new("John", 20, 170)).unwrap();
    db.insert("bob", Person::new("Bob", 30, 180)).unwrap();
    db.insert("charlie", Person::new("Jane", 25, 165)).unwrap();

    // Retrieve and print values
    hprintln!("Anne: {:?}", db.get("anne").unwrap()).unwrap();
    hprintln!("Bob: {:?}", db.get("bob").unwrap()).unwrap();
    hprintln!("Charlie: {:?}", db.get("charlie").unwrap()).unwrap();

    // Remove a key-value pair
    db.remove("bob").unwrap();
    hprintln!("bob after removal: {:?}", db.get("bob").unwrap()).unwrap();

    // exit QEMU
    // NOTE do not run this on hardware; it can corrupt OpenOCD state
    debug::exit(debug::EXIT_SUCCESS);

    loop {}
}
