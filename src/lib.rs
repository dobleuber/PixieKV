#![cfg_attr(not(test), no_std)]

extern crate alloc;

use alloc::string::String;

mod domain;

use domain::embedded_database::{EmbeddedDatabase, Database};

pub struct PixieKV<T: Sized> {
    db: EmbeddedDatabase<T>,
}

impl<T: Sized> Default for PixieKV<T> {
    fn default() -> Self {
        PixieKV { db: EmbeddedDatabase::default() }
    }
}

impl<T: Sized> Database<T> for PixieKV<T> {
    fn insert(&mut self, key: String, value: T) -> Result<(), String> {
        self.db.insert(key, value)
    }

    fn get(&self, key: &str) -> Option<&T> {
        self.db.get(key)
    }

    fn remove(&mut self, key: &str) -> Option<T> {
        self.db.remove(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut db = PixieKV::default();
        db.insert("key".to_string(), "value".to_string()).unwrap();
        assert_eq!(db.get("key"), Some(&"value".to_string()));
    }

    #[test]
    fn test_remove() {
        let mut db = PixieKV::default();
        db.insert("key".to_string(), "value".to_string()).unwrap();
        assert_eq!(db.remove("key"), Some("value".to_string()));
        assert_eq!(db.get("key"), None);
    }

    #[test]
    fn test_get() {
        let mut db = PixieKV::default();
        db.insert("key".to_string(), "value".to_string()).unwrap();
        assert_eq!(db.get("key"), Some(&"value".to_string()));
    }
}