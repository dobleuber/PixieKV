use littlefs2::{fs::Filesystem, path::Path};
use postcard::{to_slice, from_bytes};
use serde::{Serialize, de::DeserializeOwned};

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
}

impl<T: Serialize + DeserializeOwned> PixieKVStore<T> {
    pub fn save_to_file(&self, fs: &mut Filesystem<KVStorage>, filename: &str) -> Result<(), Error> {
        let mut buffer = [0u8; MAX_SIZE];
        
        let serialized_slice = to_slice(&self, &mut buffer)
            .map_err(|_| Error::Serialization)?;
        
        fs.write(Path::from_str_with_nul(filename), serialized_slice)
            .map_err(|_| Error::FileWrite)
    }

    pub fn load_from_file(fs: &mut Filesystem<KVStorage>, filename: &str) -> Result<Self, Error> {
        let contents = fs.read::<MAX_SIZE>(Path::from_str_with_nul(filename))
        .map_err(|_| Error::FileRead)?;
    
        from_bytes(&contents)
            .map_err(|_| Error::Deserialization)
    }
}
