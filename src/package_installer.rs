static SUPPORTED_MANAGERS: [&str; 3] = ["pkg.yaml", "cargo.yaml", "npm.yaml"];
static PKG_INSTALL_CMD: &str = "pkg install -y";
static CARGO_INSTALL_CMD: &str = "cargo install";
static NPM_INSTALL_CMD: &str = "npm install -g";

pub fn install(repo_path: impl AsRef<Path>) {
    fs_util::find_file_in_dir(&repo_path, SUPPORTED_MANAGERS)
        .iter()
        .map(parse_package_manager)
        .for_each(|manager| println!("Manager: {:?}", manager.info.name))
}

fn parse_package_manager(path: impl AsRef<Path>) -> PackageManager {
    parse_package_list(path)
        .map(PackageManager::from)
        .expect("Failed to parse PackageManager")
}

fn parse_package_list(path: impl AsRef<Path>) -> HashMap<String, Package> {
    fs::File::open(&cfg_map)
        .map(|file| {
            let res_map: HashMap<String, Package> =
                serde_yaml::from_reader(file).expect("failed to read cfg");
            res_map
        })
        .expect("failed to read cfg")
}

#[derive(Debug)]
enum PackageManager {
    Pkg { info: ManagerInfo },
    Cargo { info: ManagerInfo },
    Npm { info: ManagerInfo },
}

impl<T: AsRef<Path>> From<(T, HashMap<String, Package>)> for PackageManager {
    fn from((cfg_path, package_map): (T, HashMap<String, Package>)) -> Self {
        match cfg_path.as_path().file_name() {
            "pkg.yaml" => Self {
                info: ManagerInfo {
                    name: String::from("Pkg"),
                    install_command: String::from(PKG_INSTALL_CMD),
                    package_map: package_map,
                }
            },
            "cargo.yaml" => Self {
                info: ManagerInfo {
                    name: String::from("Cargo"),
                    install_command: String::from(CARGO_INSTALL_CMD)
                    package_map: package_map,
                }
            },
            "npm.yaml" => Self {
                info: ManagerInfo {
                    name: String::from("Npm"),
                    install_command: String::from(NPM_INSTALL_CMD),
                    package_map: package_map,
                }
            },
            _ => panic!("failed to create PackageManager"),
        }
    }
}


#[derive(Debug)]
struct ManagerInfo {
    name: String,
    install_command: String,
    package_map: HashMap<String, Package>,
}

struct Package {
    name: String,
    options: Option<Vector<String>>,
}
