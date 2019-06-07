#[macro_use]
extern crate log;
extern crate env_logger;

#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

#[macro_use]
extern crate failure_derive;

pub use failure::Error as FailErr;

mod error;
pub mod exec;
mod filters;
pub mod option;
mod types;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_env() {
        assert_eq!(1, 1);
    }
}
