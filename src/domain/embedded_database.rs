use heapless::FnvIndexMap as IndexMap;
use alloc::string::String;

use core::result::Result;

const MAX_SIZE: usize = 1024;

pub struct EmbeddedDatabase<T: Sized> {
    data: IndexMap<String, T, MAX_SIZE>,
}

impl<T: Sized> Default for EmbeddedDatabase<T> {
    fn default() -> Self {
        EmbeddedDatabase {
            data: IndexMap::default(),
        }
    }
}

pub trait Database<T: Sized> {
    fn insert(&mut self, key: String, value: T) -> Result<(), String>;
    fn get(&self, key: &str) -> Option<&T>;
    fn remove(&mut self, key: &str) -> Option<T>;
}

impl<T: Sized> Database<T> for EmbeddedDatabase<T> {
    fn insert(&mut self, key: String, value: T) -> Result<(), String> {
        self.data.insert(key.clone(), value )
            .map(|_| ())
            .map_err(|_| String::from("Database is full"))
    }

    fn get(&self, key: &str) -> Option<&T> {
        self.data.get(key)
    }

    fn remove(&mut self, key: &str) -> Option<T> {
        self.data.remove(key)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut db = EmbeddedDatabase::default();
        db.insert("key1".to_string(), "value1").unwrap();
        assert_eq!(db.get("key1"), Some(&"value1"));
    }

    #[test]
    fn test_remove() {
        let mut db = EmbeddedDatabase::default();
        db.insert("key1".to_string(), "value1").unwrap();
        assert_eq!(db.remove("key1"), Some("value1"));
        assert_eq!(db.get("key1"), None);
    }
}