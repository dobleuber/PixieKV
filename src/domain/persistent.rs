use littlefs2::{fs::Filesystem, path::Path};
use postcard::{to_slice, from_bytes};
use serde::{Serialize, de::DeserializeOwned};
use core::result::Result;

use crate::domain::{
    pixie_kv_store::PixieKVStore,
    storage::KVStorage,
    constants::MAX_SIZE,
};

#[derive(Debug)]
pub enum Error {
    Serialization,
    Deserialization,
    FileWrite,
    FileRead,
    IntegrityError,
}

impl<T: Serialize + DeserializeOwned> PixieKVStore<T> {
    pub fn save_to_file(&self, fs: &mut Filesystem<KVStorage>, filename: &str) -> Result<(), Error> {
        let mut buffer = [0u8; MAX_SIZE];
        
        // Serialize the data only (without integrity hash)
        let serialized_slice = to_slice(self.get_data(), &mut buffer)
            .map_err(|_| Error::Serialization)?;
        
        // Create the integrity hash filename by appending .hash
        let mut hash_filename = heapless::String::<256>::new();
        hash_filename.push_str(filename.trim_end_matches('\0')).map_err(|_| Error::Serialization)?;
        hash_filename.push_str(".hash\0").map_err(|_| Error::Serialization)?;
        
        // Save the data
        fs.write(Path::from_str_with_nul(filename), serialized_slice)
            .map_err(|_| Error::FileWrite)?;
            
        // Save the hash as a separate file
        let hash_bytes = self.get_stored_integrity_hash().to_le_bytes();
        fs.write(Path::from_str_with_nul(&hash_filename), &hash_bytes)
            .map_err(|_| Error::FileWrite)?;
            
        Ok(())
    }

    pub fn load_from_file(fs: &mut Filesystem<KVStorage>, filename: &str) -> Result<Self, Error> {
        let contents = fs.read::<MAX_SIZE>(Path::from_str_with_nul(filename))
        .map_err(|_| Error::FileRead)?;
    
        let data = from_bytes(&contents)
            .map_err(|_| Error::Deserialization)?;
            
        // Load the hash file (required)
        let mut hash_filename = heapless::String::<256>::new();
        hash_filename.push_str(filename.trim_end_matches('\0')).map_err(|_| Error::Deserialization)?;
        hash_filename.push_str(".hash\0").map_err(|_| Error::Deserialization)?;
        
        let hash_contents = fs.read::<4>(Path::from_str_with_nul(&hash_filename))
            .map_err(|_| Error::FileRead)?;
        let integrity_hash = u32::from_le_bytes([hash_contents[0], hash_contents[1], hash_contents[2], hash_contents[3]]);
            
        Ok(Self::new_with_data_and_hash(data, integrity_hash))
    }

    pub fn load_from_file_with_verification(fs: &mut Filesystem<KVStorage>, filename: &str) -> Result<Self, Error> {
        let loaded_store = Self::load_from_file(fs, filename)?;
        
        if loaded_store.verify_integrity() {
            Ok(loaded_store)
        } else {
            Err(Error::IntegrityError)
        }
    }
}
