use std::path::PathBuf;
use crate::game_file::FileStruct;

struct ModMeta {
	name: String,
	version: String,
	dependencies: Vec<*const ModMeta>,
	tags: Vec<String>,
	remote_file_id: String,
	path: String,
}

struct Mod {
	meta: ModMeta,
	content: Vec<FileStruct>,
}


impl ModMeta {
	pub fn from_file(path: PathBuf) -> Self {
		let content = std::fs::read_to_string(path).unwrap();
		Self::parse_meta(&content)
	}

	pub fn parse_meta(input: &str) -> Self {
		ModMeta {
			name: String::new(),
			version: String::new(),
			dependencies: Vec::new(),
			tags: Vec::new(),
			remote_file_id: String::new(),
			path: String::new(),
		}
	}
}