use std::path::{Path, PathBuf};

pub mod filesystem;

pub fn get_mods_metafiles<P: AsRef<Path>>(mod_dir: P) -> Vec<PathBuf> {
	match filesystem::visit_dir(mod_dir) {
		Ok(files) => files
			.iter()
			.filter_map(|file| {
				if file.extension().unwrap_or_default() == "mod" {
					Some(file.to_path_buf())
				} else {
					None
				}
			})
			.collect(),
		Err(e) => {
			eprintln!("Error visiting directory: {}", e);
			Vec::new() // 如果出错，返回空的 Vec
		}
	}
}
