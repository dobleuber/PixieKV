#![cfg_attr(not(test), no_std)]

pub mod domain;  

#[cfg(test)]
mod tests {
    use super::*;

    use domain::{
        pixie_kv::PixieKV,
        pixie_kv_store::PixieKVStore,
    };

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