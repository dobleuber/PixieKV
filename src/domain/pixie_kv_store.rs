use heapless::{String, FnvIndexMap as IndexMap};
use serde::{Serialize, Deserialize};
use core::result::Result;

use crate::domain::{
    pixie_kv::PixieKV,
    constants::{MAX_SIZE, MAX_KEY_LEN},
};

#[derive(Serialize, Deserialize)]
pub struct PixieKVStore<T: Sized> {
    data: IndexMap<String<MAX_KEY_LEN>, T, MAX_SIZE>,
}

impl<T: Sized + PartialEq  + core::cmp::Eq> PartialEq for PixieKVStore<T> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<T: Sized> Default for PixieKVStore<T> {
    fn default() -> Self {
        PixieKVStore {
            data: IndexMap::default(),
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
            PixieKVStore::default()
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
}