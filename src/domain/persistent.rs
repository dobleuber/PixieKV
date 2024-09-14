// Persistent trait to save and load data to a file
pub trait Persistent<T: Sized> {
    fn save(&self, data: T) -> Result<(), &'static str>;
    fn load(&self) -> Result<Option<T>, &'static str>;
    fn delete(&self) -> Result<Option<T>, &'static str>;
}