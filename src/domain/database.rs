pub trait Database<T: Sized> {
    fn insert(&mut self, key: &str, value: T) -> Result<(), &'static str>;
    fn get(&self, key: &str) -> Result<Option<&T>, &'static str>;
    fn remove(&mut self, key: &str) -> Result<Option<T>, &'static str>;
}