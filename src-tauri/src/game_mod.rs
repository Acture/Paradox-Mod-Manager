use crate::game_file::FileStruct;
use crate::parse::{parse_content, ParsedValue};
use dashmap::DashMap;
use serde::__private::de::Content;
use std::path::PathBuf;
use std::sync::{Arc, LazyLock, Mutex};

static MOD_MAP: LazyLock<DashMap<String, Mod>> = LazyLock::new(|| DashMap::new());

#[derive(Debug, Clone)]
struct ModMeta {
	name: String,
	version: String,
	dependencies: Vec<Arc<Mutex<ModMeta>>>,
	tags: Vec<String>,
	remote_file_id: String,
	path: String,
}

#[derive(Debug, Clone)]
struct Mod {
	meta: ModMeta,
	content: Vec<FileStruct>,
}

impl Mod {
	pub fn new() -> Self {
		Mod {
			meta: ModMeta::new(),
			content: Vec::new(),
		}
	}

	pub fn with_options(meta: Option<ModMeta>, content: Option<Vec<FileStruct>>) -> Self {
		Mod {
			meta: meta.unwrap_or_else(ModMeta::new),
			content: content.unwrap_or_else(Vec::new),
		}
	}

	pub fn from_metafile(path: PathBuf) -> (Self, Vec<Mod>) {
		let (meta, dep_mods) = ModMeta::from_file(path);
		(
			Mod {
				meta,
				content: Vec::new(),
			},
			dep_mods,
		)
	}

	pub fn load_metafile(&mut self, path: PathBuf) -> Vec<Mod> {
		let (meta, dep_mods) = ModMeta::from_file(path);
		self.meta = meta;
		dep_mods
	}

	pub fn load_content(&mut self, path: PathBuf) {
		// TODO: 从文件中加载内容
		self.content = Vec::new();
	}
}

impl ModMeta {
	pub fn new() -> Self {
		ModMeta {
			name: String::new(),
			version: String::new(),
			dependencies: Vec::new(),
			tags: Vec::new(),
			remote_file_id: String::new(),
			path: String::new(),
		}
	}

	pub fn with_options(
		name: Option<String>,
		version: Option<String>,
		dependencies: Option<Vec<Arc<Mutex<ModMeta>>>>,
		tags: Option<Vec<String>>,
		remote_file_id: Option<String>,
		path: Option<String>,
	) -> Self {
		ModMeta {
			name: name.unwrap_or_else(String::new),
			version: version.unwrap_or_else(String::new),
			dependencies: dependencies.unwrap_or_else(Vec::new),
			tags: tags.unwrap_or_else(Vec::new),
			remote_file_id: remote_file_id.unwrap_or_else(String::new),
			path: path.unwrap_or_else(String::new),
		}
	}

	pub fn from_file(path: PathBuf) -> (Self, Vec<Mod>) {
		let content = std::fs::read_to_string(path).unwrap();
		Self::parse_meta(&content)
	}

	pub fn parse_meta(input: &str) -> (Self, Vec<Mod>) {
		let res = parse_content(input);
		let mut new_deps_vec: Vec<Mod> = vec![];
		let meta = ModMeta {
			name: match res.get("name").unwrap() {
				ParsedValue::String(value) => value.clone(),
				_ => String::new(),
			},
			version: match res.get("version").unwrap() {
				ParsedValue::String(value) => value.clone(),
				_ => String::new(),
			},
			dependencies: match res.get("dependencies") {
				Some(ParsedValue::List(deps)) => deps
					.iter()
					.filter_map(|val| match val {
						ParsedValue::String(dep) => Some(MOD_MAP.get(dep).map_or_else(
							|| {
								let new_mod = Mod::with_options(
									Some(ModMeta::with_options(
										Some(dep.clone()),
										None,
										None,
										None,
										None,
										None,
									)),
									None,
								);
								let new_meta = new_mod.meta.clone();
								new_deps_vec.push(new_mod);
								Arc::new(Mutex::new(new_meta))
							},
							|mod_ref| Arc::new(Mutex::new(mod_ref.meta.clone())),
						)),
						_ => None,
					})
					.collect(),
				_ => vec![],
			},
			tags: match res.get("tags") {
				Some(ParsedValue::List(tags)) => tags
					.iter()
					.map(|tag| match tag {
						ParsedValue::String(tag_str) => tag_str.clone(),
						_ => panic!("Invalid type in 'tags' list"),
					})
					.collect(),
				_ => vec![], // 如果 tags 不存在或无效，默认为空列表
			},
			remote_file_id: match res.get("remote_file_id") {
				Some(ParsedValue::String(value)) => value.clone(),
				_ => String::new(), // 如果 remote_file_id 不存在或无效，则默认为空字符串
			},
			path: match res.get("path") {
				Some(ParsedValue::String(value)) => value.clone(),
				_ => String::new(), // 如果 path 不存在或无效，默认为空字符串
			},
		};
		(meta, new_deps_vec)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_mod_meta_from_file() {
		let (mod_meta, dep_mods) = ModMeta::from_file(PathBuf::from("tests/resources/defines.mod"));
		assert_eq!(mod_meta.name, "defines");
		assert_eq!(mod_meta.version, "0.0.1");
		assert_eq!(mod_meta.dependencies.len(), 1);
		assert_eq!(
			mod_meta.dependencies[0].lock().unwrap().name,
			"Missions Expanded"
		);
		assert_eq!(mod_meta.tags, vec!["Utilities"]);
		assert_eq!(mod_meta.remote_file_id, "");
		assert_eq!(
			mod_meta.path,
			"C:/Users/actur/Documents/Paradox Interactive/Europa Universalis IV/mod/defines"
		);
		assert_eq!(dep_mods.len(), 1);
	}

	#[test]
	fn test_mod_from_metafile() {
		let (mod_test, dep_mods) = Mod::from_metafile(PathBuf::from("tests/resources/defines.mod"));
		assert_eq!(mod_test.meta.name, "defines");
		assert_eq!(mod_test.meta.version, "0.0.1");
		assert_eq!(mod_test.meta.dependencies.len(), 1);
		assert_eq!(
			mod_test.meta.dependencies[0].lock().unwrap().name,
			"Missions Expanded"
		);
		assert_eq!(mod_test.meta.tags, vec!["Utilities"]);
		assert_eq!(mod_test.meta.remote_file_id, "");
		assert_eq!(
			mod_test.meta.path,
			"C:/Users/actur/Documents/Paradox Interactive/Europa Universalis IV/mod/defines"
		);
		assert_eq!(dep_mods.len(), 1);

		MOD_MAP.insert(mod_test.meta.name.clone(), mod_test.clone());
		dep_mods.into_iter().for_each(|dep_mod| {
			assert_eq!(dep_mod.meta.name, "Missions Expanded");
			MOD_MAP.insert(dep_mod.meta.name.clone(), dep_mod);
		});

		assert_eq!(MOD_MAP.len(), 2);
	}
}
