use std::path::{Path, PathBuf};

mod directory;
use directory::Directory;

#[derive(Debug)]
struct Core {
    root: Directory,
}

impl Core {
    fn new(path: &Path) -> std::io::Result<Self> {
        let root = Directory::build(path)?;
        Ok(Self { root })
    }
    pub fn read_home_directory() -> PathBuf {
        let environment_variable = match std::env::consts::OS {
            "windows" => "USERPROFILE",
            "macos" | "linux" => "HOME",
            _ => "",
        };
        let var_os = std::env::var_os(environment_variable);
        let home_path = match var_os {
            Some(home_path) => PathBuf::from(home_path),
            None => PathBuf::from("/"),
        };
        return home_path;
    }
}

fn main() -> std::io::Result<()> {
    let mut core = Core::new(&Path::new("/"))?;
    let home_path = Core::read_home_directory();
    core.root.append_directories_by_path(&home_path)?;
    println!("core: {:#?}", core);
    Ok(())
}
