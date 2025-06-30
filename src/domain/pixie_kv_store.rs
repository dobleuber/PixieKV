use heapless::{String, FnvIndexMap as IndexMap};
use serde::{Serialize, Deserialize};
use core::result::Result;
use crc::{Crc, CRC_32_ISO_HDLC};
use postcard;

use crate::domain::{
    pixie_kv::PixieKV,
    constants::{MAX_SIZE, MAX_KEY_LEN},
};

#[derive(Serialize, Deserialize)]
pub struct PixieKVStore<T: Sized> {
    data: IndexMap<String<MAX_KEY_LEN>, T, MAX_SIZE>,
    #[serde(skip)]
    integrity_hash: u32,
}

impl<T: Sized + PartialEq  + core::cmp::Eq> PartialEq for PixieKVStore<T> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.integrity_hash == other.integrity_hash
    }
}

impl<T: Sized> Default for PixieKVStore<T> {
    fn default() -> Self {
        PixieKVStore {
            data: IndexMap::default(),
            integrity_hash: 0,
        }
    }
}

impl<T: Sized> PixieKV<T> for PixieKVStore<T> {
    fn insert(&mut self, key: &str, value: T) -> Result<(), &'static str> {
        let heapless_key = String::<MAX_KEY_LEN>::try_from(key)
            .map_err(|_| "Key too long")?;
        self.data.insert(heapless_key, value)
            .map(|_| ())
            .map_err(|_| "Database is full")
    }

    fn get(&self, key: &str) -> Result<Option<&T>, &'static str> {
        let heapless_key = String::<MAX_KEY_LEN>::try_from(key)
            .map_err(|_| "Key too long")?;
        Ok(self.data.get(&heapless_key))
    }

    fn remove(&mut self, key: &str) -> Result<Option<T>, &'static str> {
        let heapless_key = String::<MAX_KEY_LEN>::try_from(key)
            .map_err(|_| "Key too long")?;
        Ok(self.data.remove(&heapless_key))
    }
}

impl<T: Sized + Serialize> PixieKVStore<T> {
    pub fn calculate_hash(&self) -> u32 {
        const CRC: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);
        let mut buffer = [0u8; MAX_SIZE];
        
        // Only hash the data, not the integrity_hash field to avoid recursion
        if let Ok(serialized_slice) = postcard::to_slice(&self.data, &mut buffer) {
            CRC.checksum(serialized_slice)
        } else {
            0
        }
    }

    pub fn update_integrity_hash(&mut self) {
        self.integrity_hash = self.calculate_hash();
    }

    pub fn verify_integrity(&self) -> bool {
        self.integrity_hash == self.calculate_hash()
    }

    pub fn get_integrity_hash(&self) -> u32 {
        self.integrity_hash
    }

    pub fn get_data(&self) -> &IndexMap<String<MAX_KEY_LEN>, T, MAX_SIZE> {
        &self.data
    }

    pub fn get_stored_integrity_hash(&self) -> u32 {
        self.integrity_hash
    }

    pub fn new_with_data_and_hash(data: IndexMap<String<MAX_KEY_LEN>, T, MAX_SIZE>, hash: u32) -> Self {
        PixieKVStore {
            data,
            integrity_hash: hash,
        }
    }

    pub fn insert_with_integrity(&mut self, key: &str, value: T) -> Result<(), &'static str> {
        let result = PixieKV::insert(self, key, value);
        if result.is_ok() {
            self.update_integrity_hash();
        }
        result
    }

    pub fn remove_with_integrity(&mut self, key: &str) -> Result<Option<T>, &'static str> {
        let result = PixieKV::remove(self, key);
        if result.as_ref().map_or(false, |opt| opt.is_some()) {
            self.update_integrity_hash();
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use littlefs2::fs::{Allocation, Filesystem};
    use serde::{Serialize, Deserialize};
    use crate::domain::pixie_kv_store::PixieKVStore;
    use crate::domain::storage::KVStorage;
    use crate::domain::persistent::Error;

    #[test]
    fn test_insert() {
        let mut db = PixieKVStore::default();
        db.insert("key1", "value1").unwrap();
        assert_eq!(db.get("key1"), Ok(Some(&"value1")));
    }

    #[test]
    fn test_remove() {
        let mut db = PixieKVStore::default();
        db.insert("key1", "value1").unwrap();
        assert_eq!(db.remove("key1"), Ok(Some("value1")));
        assert_eq!(db.get("key1"), Ok(None));
    }

    #[test]
    fn test_key_too_long() {
        let mut db = PixieKVStore::default();
        let long_key = "a".repeat(MAX_KEY_LEN + 1);
        assert_eq!(db.insert(&long_key, "value1"), Err("Key too long"));
    }

    #[test]
    fn test_database_full() {
        let mut db = PixieKVStore::default();
        for i in 0..MAX_SIZE {
            let key = format!("key{}", i);
            assert!(db.insert(&key, i).is_ok());
        }
        assert_eq!(db.insert("overflow", 0), Err("Database is full"));
    }

    #[test]
    fn test_get_nonexistent_key() {
        let db: PixieKVStore<i32> = PixieKVStore::default();
        assert_eq!(db.get("nonexistent"), Ok(None));
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
    struct TestValue {
        pub data: u32,
    }

    impl PixieKVStore<TestValue> {
        pub fn new() -> Self {
            let mut store = PixieKVStore::default();
            store.update_integrity_hash();
            store
        }
    }

    #[test]
    fn test_save_and_load() {
        let mut storage = KVStorage::new();
        Filesystem::format(&mut storage).unwrap();

        let alloc = &mut Allocation::new();
        let mut fs = Filesystem::mount(alloc, &mut storage).unwrap();

        let mut db = PixieKVStore::<TestValue>::new();

        db.insert("key1", TestValue { data: 100 }).unwrap();
        db.insert("key2", TestValue { data: 200 }).unwrap();

        db.save_to_file(&mut fs, "dbfile\0").unwrap();

        println!("Size of EmbeddedDatabase<TestValue>: {} bytes", std::mem::size_of::<PixieKVStore<TestValue>>());

        let loaded_db = PixieKVStore::<TestValue>::load_from_file(&mut fs, "dbfile\0").unwrap();

        let value1 = loaded_db.get("key1").unwrap().unwrap();
        let value2 = loaded_db.get("key2").unwrap().unwrap();

        assert_eq!(value1.data, 100);
        assert_eq!(value2.data, 200);
    }

    #[test]
    fn test_save_to_file_error() {
        let mut storage = KVStorage::new();
        let alloc = &mut Allocation::new();
        let fs_result = Filesystem::mount(alloc, &mut storage);

        assert!(fs_result.is_err());
    }

    #[test]
    fn test_load_from_file_error() {
        let mut storage = KVStorage::new();
        let alloc = &mut Allocation::new();
        Filesystem::format(&mut storage).unwrap();
        let mut fs = Filesystem::mount(alloc, &mut storage).unwrap();

        let result = PixieKVStore::<TestValue>::load_from_file(&mut fs, "nonexistent\0");

        assert!(matches!(result, Err(Error::FileRead)));
    }

    #[test]
    fn test_integrity_hash_calculation() {
        let mut db = PixieKVStore::<TestValue>::new();
        let initial_hash = db.get_integrity_hash();

        db.insert_with_integrity("key1", TestValue { data: 100 }).unwrap();
        let hash_after_insert = db.get_integrity_hash();

        assert_ne!(initial_hash, hash_after_insert);
        assert!(db.verify_integrity());
    }

    #[test]
    fn test_integrity_verification() {
        let mut db = PixieKVStore::<TestValue>::new();
        db.insert_with_integrity("key1", TestValue { data: 100 }).unwrap();
        
        assert!(db.verify_integrity());
    }

    #[test]
    fn test_integrity_after_operations() {
        let mut db = PixieKVStore::<TestValue>::new();
        
        db.insert_with_integrity("key1", TestValue { data: 100 }).unwrap();
        assert!(db.verify_integrity());

        db.insert_with_integrity("key2", TestValue { data: 200 }).unwrap();
        assert!(db.verify_integrity());

        db.remove_with_integrity("key1").unwrap();
        assert!(db.verify_integrity());
        
        db.remove_with_integrity("key2").unwrap();
        assert!(db.verify_integrity());
    }

    #[test]
    fn test_save_and_load_with_verification() {
        let mut storage = KVStorage::new();
        Filesystem::format(&mut storage).unwrap();

        let alloc = &mut Allocation::new();
        let mut fs = Filesystem::mount(alloc, &mut storage).unwrap();

        let mut db = PixieKVStore::<TestValue>::new();
        db.insert_with_integrity("key1", TestValue { data: 100 }).unwrap();
        db.insert_with_integrity("key2", TestValue { data: 200 }).unwrap();

        db.save_to_file(&mut fs, "dbfile_verified\0").unwrap();

        let loaded_db = PixieKVStore::<TestValue>::load_from_file_with_verification(&mut fs, "dbfile_verified\0").unwrap();

        assert_eq!(loaded_db.get("key1").unwrap().unwrap().data, 100);
        assert_eq!(loaded_db.get("key2").unwrap().unwrap().data, 200);
        assert!(loaded_db.verify_integrity());
    }

    #[test]
    fn test_hash_consistency() {
        let mut db1 = PixieKVStore::<TestValue>::new();
        let mut db2 = PixieKVStore::<TestValue>::new();

        db1.insert_with_integrity("key1", TestValue { data: 100 }).unwrap();
        db2.insert_with_integrity("key1", TestValue { data: 100 }).unwrap();

        assert_eq!(db1.get_integrity_hash(), db2.get_integrity_hash());
    }
}