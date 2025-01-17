mod config;
mod file;
use dashmap::DashMap;
use std::sync::LazyLock;
use log::{info, debug};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
	format!("Hello, {}! You've been greeted from Rust!", name)
}

static GAME_CONFIG: LazyLock<DashMap<String, config::GameConfig>> =
	LazyLock::new(|| DashMap::new());

fn load_config(){
	let config = config::Config::default();

	if config.config_save_path.exists() {
		info!("Loading config from {:?}", config.config_save_path);

		let content = match std::fs::read(&config.config_save_path) {
			Ok(c) => c,
			Err(e) => {
				eprintln!("Error reading file {:?}: {}", config.config_save_path, e);
				return;
			}
		};
		let game_configs: Vec<config::GameConfig> = match serde_json::from_slice(&content) {
			Ok(gc) => gc,
			Err(e) => {
				eprintln!("Error parsing config file {:?}: {}", config.config_save_path, e);
				return;
			}
		};
		for game_config in game_configs {
			GAME_CONFIG.insert(game_config.game_name.clone(), game_config);
		}
	} else {
		info!("Config file {:?} not found, using default config", config.config_save_path);
	}
}

#[tauri::command]
fn setup_game_config(game_name: &str, game_dir: &str, mod_dir: &str) {
	let game_dir = std::path::PathBuf::from(game_dir);
	let mod_dir = std::path::PathBuf::from(mod_dir);
	let game_config = config::GameConfig::new(game_name.to_string(), game_dir, mod_dir);
	GAME_CONFIG.insert(game_name.to_string(), game_config);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
	tauri::Builder::default()
			.plugin(tauri_plugin_shell::init())
			.invoke_handler(tauri::generate_handler![greet])
			.run(tauri::generate_context!())
			.expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
	use log::LevelFilter;
	use super::*;

	#[test]
	fn test_greet() {
		assert_eq!(
			greet("world"),
			"Hello, world! You've been greeted from Rust!"
		);
	}

	#[test]
	fn test_load_config(){
		env_logger::builder().filter_level(LevelFilter::Info).init();
		load_config();
	}

	#[test]
	fn test_setup_game_config() {
		setup_game_config("test", ".", "src");
		let binding = GAME_CONFIG.get("test").unwrap();
		let game_config = binding.value();
		println!("{:?}", game_config);
	}
}
