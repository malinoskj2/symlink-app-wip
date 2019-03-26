/// The enum holding the various errors used in the crate
#[derive(Debug, Fail)]
pub enum InstallerErr {
    #[fail(display = "Failed to resolve PATH.")]
    NoPath,
    #[fail(display = "Failed to convert to str from OsStr")]
    StringConversionFail,
    #[fail(display = "Failed to create symlink")]
    SymLinkFail,
    #[fail(display = "Failed to create manager schema")]
    SchemaBuildError,
    #[fail(display = "yaml parse error")]
    YamlParseFail,
    #[fail(display = "tried to execute install with no packages")]
    NoPackageInstallError,
    #[fail(display = "shell execution error")]
    ShellExecutionFail,
    #[fail(display = "Could not find config-map")]
    NoConfigFound,
}
