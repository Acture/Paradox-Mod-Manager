use std::path::PathBuf;

#[derive(Debug)]
pub struct GameConfig {
	game_name: String,
	game_dir: PathBuf,
	mod_dir: PathBuf,
}

impl GameConfig {
	pub(crate) fn new(game_name: String, game_dir: PathBuf, mod_dir: PathBuf) -> Self {
		Self {
			game_name,
			game_dir,
			mod_dir,
		}
	}

	pub fn is_game_dir_exists(&self) -> bool {
		self.game_dir.exists()
	}

	pub fn is_mod_dir_exists(&self) -> bool {
		self.mod_dir.exists()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_config_struct() {
		let config = GameConfig::new("test".to_string(), PathBuf::from("."), PathBuf::from("src"));
		assert!(config.is_game_dir_exists());
		assert!(config.is_mod_dir_exists());
	}
}
