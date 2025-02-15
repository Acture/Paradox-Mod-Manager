use crate::utility::filesystem::visit_dir;
use derive_builder::Builder;
use md5::{Digest, Md5};
use std::error::Error;
use std::fs::File;
use std::io::{Read, BufReader};
use std::path::{Path, PathBuf};

#[derive(Builder, Debug, Clone)]
pub struct Component {
	name: String,
	path: PathBuf,
	hash: [u8; 16],
}

impl Component {
	pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Component, Box<dyn Error>> {
		let path_ref = path.as_ref();

		// 处理文件名提取
		let file_name = path_ref
			.file_name()
			.ok_or("Invalid file name")?
			.to_str()
			.ok_or("Invalid UTF-8 in file name")?
			.to_string();

		// 打开文件
		let file = File::open(path_ref)?;
		let mut reader = BufReader::new(file);

		let mut hasher = Md5::new();
		let mut buffer = Vec::new();

		// 逐字节读取并更新哈希
		reader.read_to_end(&mut buffer)?;
		hasher.update(&buffer);

		// 计算最终哈希值
		let hash = hasher.finalize();

		// 构建组件
		Ok(ComponentBuilder::default()
			.name(file_name)
			.path(path_ref.to_path_buf())
			.hash(hash.into())
			.build()?)
	}

	pub fn get_components_from_dir<P: AsRef<Path>>(
		dir: P,
	) -> Result<Vec<Component>, Box<dyn Error>> {
		let mut components = Vec::new();
		let files = visit_dir(&dir)?;

		for file in files {
			let component = Component::from_path(file);
			match component {
				Ok(component) => components.push(component),
				Err(e) => eprintln!(
					"Error getting component from file {} from dir {}",
					e,
					dir.as_ref().display()
				),
			}
		}

		Ok(components)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_component_dir() {
		let test_dir =
			Path::new("D:/actur/Documents/Paradox Interactive/Europa Universalis IV/mod/defines/");
		assert!(test_dir.exists());
		assert!(test_dir.is_dir());
		//
		// let components = Component::get_components_from_dir(test_dir).unwrap();
		// println!("{:?}", components);
	}
}
