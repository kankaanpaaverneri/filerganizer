use std::path::{Component, Path, PathBuf};

#[derive(Debug, Clone)]
pub struct Directory {
    path: PathBuf,
    directories: Vec<Directory>,
    files: Vec<PathBuf>,
}

impl Directory {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            directories: Vec::new(),
            files: Vec::new(),
        }
    }
    pub fn build(path: &Path) -> std::io::Result<Self> {
        let (directories, files) = Self::read_directory(path)?;
        Ok(Self {
            path: PathBuf::from(path),
            directories,
            files,
        })
    }
    // Returns directories and files of a given path from the filesystem
    pub fn read_directory(path: &Path) -> std::io::Result<(Vec<Directory>, Vec<PathBuf>)> {
        let read_dir = std::fs::read_dir(path)?;
        let mut directories = Vec::new();
        let mut files = Vec::new();
        for entry_result in read_dir {
            let entry = entry_result?;

            match entry.file_type() {
                Ok(file_type) => {
                    if file_type.is_dir() {
                        directories.push(Directory::new(entry.path()));
                    } else if file_type.is_file() {
                        files.push(entry.path());
                    }
                }
                Err(_) => continue,
            }
        }
        Ok((directories, files))
    }
    // Appends more directories from the filesystem
    pub fn read_child_directories(&mut self) -> std::io::Result<()> {
        if !self.directories.is_empty() || !self.files.is_empty() {
            return Ok(());
        }
        let (directories, files) = Directory::read_directory(&self.path)?;
        self.directories = directories;
        self.files = files;
        Ok(())
    }

    pub fn append_directories_by_path(
        &mut self,
        target_path: &Path,
    ) -> std::io::Result<Option<&mut Directory>> {
        // Filter out root directory from the target_path
        let mut components = target_path.components().filter(|c| match c {
            Component::RootDir => false,
            _ => true,
        });

        let mut current_directory = self;

        // Iterate target_path components
        while let Some(component) = components.next() {
            let name = component.as_os_str();
            let mut found = None;
            // Iterate current directory's subdirectories to find same directory name as the component
            for directory in &mut current_directory.directories {
                if directory.path.file_name() == Some(name) {
                    found = Some(directory);
                    break;
                }
            }

            // If found current directory becomes the found subdirectory
            match found {
                Some(child_directory) => current_directory = child_directory,
                None => return Ok(None),
            }
            // Append more sub directories from the filesystem
            current_directory.read_child_directories()?;
        }

        Ok(Some(current_directory))
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use tempfile;

    use super::*;

    #[test]
    fn test_append_directories_by_path() -> std::io::Result<()> {
        let temp_dir = tempfile::tempdir()?;
        std::fs::create_dir_all(temp_dir.path().join("a/b/c"))?;
        let file_path = temp_dir.path().join("a/b/c/mytempfile.txt");
        File::create_new(file_path)?;

        let mut root = Directory::build(temp_dir.path())?;
        let node = root.append_directories_by_path(temp_dir.path().join("/a").as_path())?;
        assert_eq!(node.unwrap().files.len(), 0);
        let node = root.append_directories_by_path(temp_dir.path().join("/a/b/c").as_path())?;
        assert_eq!(node.unwrap().files.len(), 1);
        Ok(())
    }

    #[test]
    fn test_read_directory() -> std::io::Result<()> {
        let temp_dir = tempfile::tempdir()?;
        std::fs::create_dir_all(temp_dir.path().join("a/b"))?;
        let file_path = temp_dir.path().join("a/mytempfile.txt");
        File::create_new(file_path)?;
        let (directories, files) = Directory::read_directory(&temp_dir.path().join("a"))?;
        assert_eq!(files.len(), 1);
        assert_eq!(directories.len(), 1);
        Ok(())
    }
}
