use heapless::FnvIndexMap as IndexMap;
use heapless::String;
use serde::{Serialize, Deserialize};

use core::result::Result;

use crate::domain::{
    database::Database,
    constants::{MAX_SIZE, MAX_KEY_LEN},
};

#[derive(Serialize, Deserialize)]
pub struct EmbeddedDatabase<T: Sized> {
    data: IndexMap<String<MAX_KEY_LEN>, T, MAX_SIZE>,
}

impl<T: Sized + PartialEq  + core::cmp::Eq> PartialEq for EmbeddedDatabase<T> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<T: Sized> Default for EmbeddedDatabase<T> {
    fn default() -> Self {
        EmbeddedDatabase {
            data: IndexMap::default(),
        }
    }
}

impl<T: Sized> Database<T> for EmbeddedDatabase<T> {
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

    #[test]
    fn test_insert() {
        let mut db = EmbeddedDatabase::default();
        db.insert("key1", "value1").unwrap();
        assert_eq!(db.get("key1"), Ok(Some(&"value1")));
    }

    #[test]
    fn test_remove() {
        let mut db = EmbeddedDatabase::default();
        db.insert("key1", "value1").unwrap();
        assert_eq!(db.remove("key1"), Ok(Some("value1")));
        assert_eq!(db.get("key1"), Ok(None));
    }

    #[test]
    fn test_key_too_long() {
        let mut db = EmbeddedDatabase::default();
        let long_key = "a".repeat(MAX_KEY_LEN + 1);
        assert_eq!(db.insert(&long_key, "value1"), Err("Key too long"));
    }

    #[test]
    fn test_database_full() {
        let mut db = EmbeddedDatabase::default();
        for i in 0..MAX_SIZE {
            let key = format!("key{}", i);
            assert!(db.insert(&key, i).is_ok());
        }
        assert_eq!(db.insert("overflow", 0), Err("Database is full"));
    }

    #[test]
    fn test_get_nonexistent_key() {
        let db: EmbeddedDatabase<i32> = EmbeddedDatabase::default();
        assert_eq!(db.get("nonexistent"), Ok(None));
    }   
}