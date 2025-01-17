use md5;
use std::path::PathBuf;

#[derive(Debug)]
pub struct FileStruct {
    name: String,
    base_dir: PathBuf,
    relative_path: PathBuf,
    hash: Option<[u8; 16]>,
}

impl FileStruct {
    fn new(name: String, base_dir: PathBuf, relative_path: PathBuf) -> Self {
        Self {
            name,
            base_dir,
            relative_path,
            hash: None,
        }
    }
    fn calculate_hash(&mut self) {
        let path = self.base_dir.join(&self.relative_path);
        let content = match std::fs::read(&path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Error reading file {:?}: {}", path, e);
                return;
            }
        };
        let hasher = md5::compute(content);
        self.hash = Some(hasher.into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_struct() {
        let mut file = FileStruct::new(
            "test".to_string(),
            PathBuf::from("."),
            PathBuf::from("Cargo.toml"),
        );
        file.calculate_hash();
        println!("{:?}", file.hash);
        assert!(file.hash.is_some());
    }
}
