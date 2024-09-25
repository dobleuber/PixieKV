#![cfg_attr(not(test), no_std)]

mod domain;
use domain::embedded_database::EmbeddedDatabase;

pub type PixieKVStore<T> = EmbeddedDatabase<T>;

#[cfg(test)]
mod tests {
    use super::*;

    use domain::database::PixieKV;

    #[test]
    fn test_insert() {
        let mut db = PixieKVStore::default();
        db.insert("key", "value".to_string()).unwrap();
        assert_eq!(db.get("key"), Ok(Some(&"value".to_string())));
    }

    #[test]
    fn test_remove() {
        let mut db = PixieKVStore::default();
        db.insert("key", "value".to_string()).unwrap();
        assert_eq!(db.remove("key"), Ok(Some("value".to_string())));   
        assert_eq!(db.get("key"), Ok(None));
    }

    #[test]
    fn test_get() {
        let mut db = PixieKVStore::default();
        db.insert("key", "value".to_string()).unwrap();
        assert_eq!(db.get("key"), Ok(Some(&"value".to_string())));
    }
}