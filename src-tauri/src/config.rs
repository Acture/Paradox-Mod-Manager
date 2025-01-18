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

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub(crate) struct GameConfig {
    pub(crate) game_name: String,
    pub(crate) game_dir: PathBuf,
    pub(crate) mod_dir: PathBuf,
}

impl GameConfig {
    pub(crate) fn new(game_name: String, game_dir: PathBuf, mod_dir: PathBuf) -> Self {
        Self {
            game_name,
            game_dir,
            mod_dir,
        }
    }

    pub(crate) fn is_game_dir_exists(&self) -> bool {
        self.game_dir.exists()
    }

    pub(crate) fn is_mod_dir_exists(&self) -> bool {
        self.mod_dir.exists()
    }

    pub(crate) fn validate(&self) -> Result<(), String> {
        if !self.is_game_dir_exists() {
            return Err(format!("Game directory {:?} does not exist", self.game_dir));
        }
        if !self.is_mod_dir_exists() {
            return Err(format!("Mod directory {:?} does not exist", self.mod_dir));
        }
        Ok(())
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
