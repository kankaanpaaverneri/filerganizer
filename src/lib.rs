use std::path::{Path, PathBuf};

mod directory;
use directory::Directory;

#[derive(Debug)]
pub struct Core {
    pub root: Directory,
    pub selected_files: Vec<PathBuf>,
}

pub enum Message {
    Append(PathBuf),
}

impl Core {
    pub fn new(path: &Path) -> std::io::Result<Self> {
        let root = Directory::build(path)?;
        Ok(Self {
            root,
            selected_files: Vec::new(),
        })
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

    pub fn update(&mut self, message: Message) -> std::io::Result<()> {
        match message {
            Message::Append(new_path) => {
                self.root.append_directories_by_path(&new_path)?;

                Ok(())
            }
        }
    }
}
