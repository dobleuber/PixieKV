#![cfg_attr(not(test), no_std)]

extern crate alloc;

mod domain;

use domain::embedded_database::EmbeddedDatabase;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}