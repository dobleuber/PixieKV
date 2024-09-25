use pixie_kv::domain::pixie_kv_store::PixieKVStore;
use pixie_kv::domain::pixie_kv::PixieKV;
use pixie_kv::domain::persistent::Error;
use littlefs2::fs::{Filesystem, Allocation};
use pixie_kv::domain::storage::KVStorage;
use heapless::String;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Person <const N: usize>{
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

fn main() -> Result<(), Error> {
    println!("PixieKV Demo");

    // Create a new PixieKVStore
    let mut db: PixieKVStore<Person32> = PixieKVStore::default();

    // Insert some key-value pairs
    db.insert("anne", Person::new("John", 20, 170)).unwrap();
    db.insert("bob", Person::new("Bob", 30, 180)).unwrap();
    db.insert("charlie", Person::new("Jane", 25, 165)).unwrap();

    // Retrieve and print values
    println!("Anne: {:?}", db.get("anne").unwrap());
    println!("Bob: {:?}", db.get("bob").unwrap());
    println!("Charlie: {:?}", db.get("charlie").unwrap());

    // Remove a key-value pair
    db.remove("bob").unwrap();
    println!("bob after removal: {:?}", db.get("bob").unwrap());

    // Save the database to a file
    let mut storage = KVStorage::new();
    Filesystem::format(&mut storage).unwrap();
    let alloc = &mut Allocation::new();
    let mut fs = Filesystem::mount(alloc, &mut storage).unwrap();
    
    db.save_to_file(&mut fs, "database.db\0")?;
    println!("Database saved to file");

    // Load the database from the file
    let loaded_db: PixieKVStore<Person32> = PixieKVStore::load_from_file(&mut fs, "database.db\0")?;
    println!("Loaded Person 1: {:?}", loaded_db.get("anne").unwrap());
    println!("Loaded Person 2: {:?}", loaded_db.get("charlie").unwrap());

    Ok(())
}