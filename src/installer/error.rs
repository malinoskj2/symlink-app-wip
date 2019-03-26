/// The enum holding the various errors used in the crate
#[derive(Debug, Fail)]
pub enum SSErr {
    #[fail(display = "Failed to resolve PATH.")]
    NoPath,
}
