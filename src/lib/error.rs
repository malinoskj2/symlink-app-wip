/// The enum holding the various errors used in the crate
#[derive(Debug, Fail)]
pub enum InstallerErr {
    #[fail(display = "yaml parse error")]
    YamlParseFail(#[cause] serde_yaml::Error),
    #[fail(display = "yaml parse error IO")]
    IOYamlParseFail(#[cause] std::io::Error),
}

impl From<serde_yaml::Error> for InstallerErr {
    fn from(err: serde_yaml::Error) -> Self {
        InstallerErr::YamlParseFail(err)
    }
}

impl From<std::io::Error> for InstallerErr {
    fn from(err: std::io::Error) -> Self {
        InstallerErr::IOYamlParseFail(err)
    }
}
