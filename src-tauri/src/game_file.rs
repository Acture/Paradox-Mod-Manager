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

pub fn get_files(base_dir: PathBuf) -> Vec<FileStruct> {
	let mut files = Vec::new();
	for entry in std::fs::read_dir(&base_dir).unwrap() {
		let entry = entry.unwrap();
		let path = entry.path();
		if path.is_dir() {
			continue;
		}
		let name = entry.file_name().to_string_lossy().to_string();
		let relative_path = path.strip_prefix(&base_dir).unwrap().to_path_buf();
		let mut file = FileStruct::new(name, base_dir.clone(), relative_path);
		file.calculate_hash();
		files.push(file);
	}
	files
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

	#[test]
	fn test_get_files() {
		let files = get_files(PathBuf::from("."));
		println!("{:?}", files);
	}
}
