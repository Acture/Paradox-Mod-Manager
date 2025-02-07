use std::path::PathBuf;

#[derive(Debug)]
pub(crate) struct Config {
	pub(crate) config_save_path: PathBuf,
}

impl Config {
	pub(crate) fn new(config_save_path: PathBuf) -> Self {
		Self { config_save_path }
	}
	pub(crate) fn default() -> Self {
		Self {
			config_save_path: PathBuf::from("config.yaml"),
		}
	}
}



