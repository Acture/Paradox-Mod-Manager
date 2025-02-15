use std::path::{Path, PathBuf};
use std::{fs, io};

pub fn visit_dir<P: AsRef<Path>>(path: P) -> Result<Vec<PathBuf>, io::Error> {
	if !path.as_ref().exists() {
		return Err(io::Error::new(
			io::ErrorKind::NotFound,
			format!("Path {} does not exist", path.as_ref().display()),
		));
	}


	let mut files = Vec::new();
	let mut dirs = vec![path.as_ref().to_path_buf()];

	// 迭代遍历栈中的目录
	while let Some(dir_path) = dirs.pop() {
		let dir = fs::read_dir(&dir_path)?;

		for entry in dir {
			let entry = entry?;
			let entry_path = entry.path();
			let metadata = fs::metadata(&entry_path)?;

			if metadata.is_dir() {
				dirs.push(entry_path);
			} else {
				files.push(entry_path);
			}
		}
	}

	Ok(files)
}
