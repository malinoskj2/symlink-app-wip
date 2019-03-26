#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;
#[macro_use]
extern crate derive_builder;
#[macro_use]
extern crate dotenv_codegen;
extern crate dotenv;

#[macro_use]
extern crate failure_derive;

#[macro_use]
extern crate lazy_static;

pub use failure::Error as FailErr;

pub mod config_linker;
mod error;
mod fs_util;
pub mod package_installer;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_env() {
        assert_eq!("test_value", dotenv!("TEST_ENV_PROPERTY"));
    }
}
