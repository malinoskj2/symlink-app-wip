use super::fs_util;
use super::FailErr;
use dotenv::dotenv;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use subprocess::{Exec, ExitStatus, Redirection};

pub fn install(repo_path: impl AsRef<Path>) {
    let config_names = vec![
        "pkg.yaml".to_string(),
        "cargo.yaml".to_string(),
        "npm.yaml".to_string(),
    ];

    let config_paths = fs_util::find_file_in_dir(repo_path.as_ref(), config_names);

    config_paths
        .into_iter()
        .map(parse_package_manager)
        .for_each(|manager| {
            manager
                .install_packages()
                .expect("failed to install packages")
        });
}

fn parse_package_manager(path: impl AsRef<Path>) -> PackageManager {
    let packages = parse_packages(path.as_ref());
    let parse_name: String = path
        .as_ref()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned()
        .split('.')
        .take(1)
        .collect();

    let cmd = get_cmds()
        .get(parse_name.as_str())
        .expect("failed to get")
        .to_owned();

    PackageManagerBuilder::default()
        .name(parse_name)
        .install_command(cmd.to_owned())
        .packages(parse_packages(path.as_ref()))
        .build()
        .expect("failed to build manager")
}

fn parse_packages(path: impl AsRef<Path>) -> Vec<Package> {
    fs::File::open(&path)
        .map(|file| {
            let res_map: Vec<Package> = serde_yaml::from_reader(file).expect("failed to read cfg");
            res_map
        })
        .expect("failed to read cfg")
}

fn get_cmds() -> HashMap<String, String> {
    let mut map = HashMap::with_capacity(3);
    map.entry("pkg".to_string())
        .or_insert_with(|| "sudo pkg install -y".to_string());
    map.entry("cargo".to_string())
        .or_insert_with(|| "cargo install".to_string());
    map.entry("npm".to_string())
        .or_insert_with(|| "npm install -g".to_string());
    map
}

#[derive(Default, Builder, Debug)]
#[builder(setter(into))]
struct PackageManager {
    name: String,
    install_command: String,
    packages: Vec<Package>,
}

impl PackageManager {
    fn generate_install_string(&self) -> String {
        let pack_list: String = self
            .packages
            .iter()
            .map(|package| format!("{} ", package.name))
            .collect();

        format!("{} {}", self.install_command, pack_list)
    }

    fn install_packages(&self) -> Result<(), FailErr> {
        println!("Installing Packages for: {}", self.name);

        println!("Install String");
        let output = Exec::shell(self.generate_install_string())
            .stdout(Redirection::Pipe)
            .stderr(Redirection::Merge)
            .capture()?
            .stdout_str();

        println!("{}", output);
        println!("finished installing packages for: {}", self.name);
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Package {
    name: String,
    options: Option<Vec<String>>,
}
