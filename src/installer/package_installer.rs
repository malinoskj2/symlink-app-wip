use super::error::InstallerErr;
use super::fs_util;
use super::FailErr;
use crate::fs_util::{ff_in_dir, find_file_in_dir};
use dotenv::dotenv;
use failure::Fail;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use subprocess::{Exec, ExitStatus, Redirection};

pub fn install(repo_path: impl AsRef<Path>) -> Result<(), FailErr> {
    let manager_schemas = init_manager_schemas()?;

    let managers: Vec<PackageManager> = manager_schemas
        .into_iter()
        .filter_map(|schema| PackageManager::try_from_schema(repo_path.as_ref(), schema).ok())
        .collect();

    managers
        .iter()
        .for_each(|manager| println!("Manager: {:?}", manager));

    Ok(())
}

fn init_manager_schemas() -> Result<Vec<ManagerSchema>, FailErr> {
    let pkg = ManagerSchemaBuilder::default()
        .name(String::from("Pkg"))
        .install_command(dotenv!("PKG_INSTALL_COMMAND"))
        .config_name(dotenv!("PKG_CONFIG_NAME"))
        .privileged(dotenv!("PKG_PRIV") == "YES")
        .build()
        .map_err(|_| InstallerErr::SchemaBuildError)?;

    let cargo = ManagerSchemaBuilder::default()
        .name(String::from("Cargo"))
        .install_command(dotenv!("CARGO_INSTALL_COMMAND"))
        .config_name(dotenv!("CARGO_CONFIG_NAME"))
        .privileged(dotenv!("CARGO_PRIV") == "YES")
        .build()
        .map_err(|_| InstallerErr::SchemaBuildError)?;

    let npm = ManagerSchemaBuilder::default()
        .name(String::from("Npm"))
        .install_command(dotenv!("NPM_INSTALL_COMMAND"))
        .config_name(dotenv!("NPM_CONFIG_NAME"))
        .privileged(dotenv!("NPM_PRIV") == "YES")
        .build()
        .map_err(|_| InstallerErr::SchemaBuildError)?;

    Ok(vec![pkg, cargo, npm])
}

#[derive(Default, Builder, Debug)]
#[builder(setter(into))]
struct ManagerSchema {
    name: String,
    install_command: String,
    config_name: String,
    privileged: bool,
}

#[derive(Default, Builder, Debug)]
#[builder(setter(into))]
struct PackageManager {
    name: String,
    install_command: String,
    config_name: String,
    packages: Option<Vec<Package>>,
    config_path: Option<PathBuf>,
    root: bool,
}

impl PackageManager {
    fn generate_install_string(&self) -> Option<String> {
        let list: Option<String> = self.packages.as_ref().map(|packages| {
            packages
                .iter()
                .map(|p_name| format!("{:?} ", p_name))
                .collect()
        });

        let priv_string = if self.root { "sudo " } else { "" };

        list.map(|pack_list| format!("{}{} {}", priv_string, self.install_command, pack_list))
    }

    fn install_packages(&self) -> Result<(), FailErr> {
        let install_string = self
            .generate_install_string()
            .ok_or_else(|| InstallerErr::NoPackageInstallError)?;

        let output = Exec::shell(install_string)
            .stdout(Redirection::Pipe)
            .stderr(Redirection::Merge)
            .capture()
            .map_err(|_| InstallerErr::ShellExecutionFail)?
            .stdout_str();

        Ok(())
    }

    fn try_from_schema(
        path: impl AsRef<Path>,
        schema: ManagerSchema,
    ) -> Result<PackageManager, FailErr> {
        let mut res_dir: Vec<PathBuf> =
            find_file_in_dir(path.as_ref(), vec![schema.config_name.clone()])?;

        let res_dir: PathBuf = res_dir.remove(0);
        let packages = Self::parse_packages(res_dir.to_path_buf())?;

        PackageManagerBuilder::default()
            .name(schema.name)
            .install_command(schema.install_command)
            .config_name(schema.config_name)
            .packages(packages)
            .config_path(res_dir)
            .root(schema.privileged)
            .build()
            .map_err(|_| InstallerErr::SchemaBuildError)
            .map(Ok)?
    }

    fn parse_packages(cfg_map: impl AsRef<Path>) -> Result<Vec<Package>, FailErr> {
        Ok(fs::File::open(&cfg_map)
            .map_err(|_| InstallerErr::NoPackageInstallError)
            .map(|file| {
                let res_map: Vec<Package> =
                    serde_yaml::from_reader(file).expect("failed to read cfg");
                res_map
            })
            .expect("couldnt parse packages"))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Package {
    name: String,
    options: Option<Vec<String>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_env() -> Result<(), FailErr> {
        let schemas = init_manager_schemas()?;

        assert_eq!(3, schemas.len());
        schemas.iter().for_each(|schema| {
            assert!(schema.install_command.len() > 0);
            assert!(schema.config_name.len() > 0);
        });
        Ok(())
    }
}
