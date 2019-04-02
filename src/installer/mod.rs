#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;
#[macro_use]
extern crate derive_builder;

#[macro_use]
extern crate failure_derive;

pub use failure::Error as FailErr;

pub mod config_linker;
mod error;
mod fs_util;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_env() {
        assert_eq!(1, 1);
    }
}
