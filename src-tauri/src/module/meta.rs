use crate::lang::parser::parse_content;
use derive_builder::Builder;
use std::path::Path;

#[derive(Builder, Debug, Clone)]
pub struct Meta {
	pub name: String,
	pub version: Option<String>,
	pub local_path: String,
	pub dependencies: Vec<String>,
}

impl Meta {
	pub fn new(
		name: String,
		version: String,
		local_path: String,
		dependencies: Vec<String>,
	) -> Meta {
		Meta {
			name,
			version: Some(version),
			local_path,
			dependencies,
		}
	}

	pub fn create_from_mod_file<P: AsRef<Path>>(
		file_path: P,
	) -> Result<Meta, Box<dyn std::error::Error>> {
		let content = std::fs::read_to_string(&file_path)?;
		let parsed = parse_content(&content)?;

		Ok(MetaBuilder::default()
			.name(parsed.get("name").unwrap().as_string().unwrap())
			.version(match parsed.get("version") {
				Some(version) => Some(version.as_string().unwrap()),
				None => None,
			})
			.local_path(match parsed.get("path") {
				Some(path) => path.as_string().unwrap(),
				None => file_path.as_ref().to_str().unwrap().to_string(),
			})
			.dependencies(match parsed.get("dependencies") {
				Some(deps) => deps
					.as_array()
					.unwrap()
					.iter()
					.map(|dep| dep.as_string().unwrap().to_string())
					.collect(),
				None => Vec::new(),
			})
			.build()
			.unwrap())
	}
}
