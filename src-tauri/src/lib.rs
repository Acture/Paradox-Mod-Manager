mod config;
mod file;
use dashmap::DashMap;
use log::{debug, error, info};
use serde_json;
use serde_yaml;
use std::sync::LazyLock;
use tauri::{Manager, RunEvent};
use tauri_plugin_log::{Target, TargetKind};

static GAME_CONFIG: LazyLock<DashMap<String, config::GameConfig>> =
	LazyLock::new(|| DashMap::new());

static CONFIG: LazyLock<config::Config> = LazyLock::new(|| config::Config::default());

fn load_game_config() {
	if CONFIG.config_save_path.exists() {
		info!("Loading config from {:?}", CONFIG.config_save_path);

		let content = match std::fs::read(&CONFIG.config_save_path) {
			Ok(c) => c,
			Err(e) => {
				error!("Error reading file {:?}: {}", CONFIG.config_save_path, e);
				return;
			}
		};
		let game_configs: Vec<config::GameConfig> = match serde_yaml::from_slice(&content) {
			Ok(gc) => gc,
			Err(e) => {
				error!(
					"Error parsing config file {:?}: {}",
					CONFIG.config_save_path, e
				);
				return;
			}
		};
		for game_config in game_configs {
			GAME_CONFIG.insert(game_config.game_name.clone(), game_config);
		}
	} else {
		info!(
			"Config file {:?} not found, using default config",
			CONFIG.config_save_path
		);
	}
	info!("Loaded {} game configurations", GAME_CONFIG.len());
	GAME_CONFIG.iter().for_each(|entry| {
		debug!("Key: {}, Value: {:?}", entry.key(), entry.value());
	});
}

fn save_game_config() {
	let game_configs: Vec<config::GameConfig> =
		GAME_CONFIG.iter().map(|gc| gc.value().clone()).collect();
	if game_configs.is_empty() {
		info!("No game configurations to save.");
		return; // 如果配置为空，直接返回，跳过保存
	}

	let content = match serde_yaml::to_string(&game_configs) {
		Ok(c) => c,
		Err(e) => {
			error!("Error serializing game configs: {}", e);
			return;
		}
	};
	match std::fs::write(&CONFIG.config_save_path, content) {
		Ok(_) => info!("Config saved to {:?}", CONFIG.config_save_path),
		Err(e) => error!(
			"Error saving config to {:?}: {}",
			CONFIG.config_save_path, e
		),
	}
}

#[tauri::command(rename_all = "snake_case")]
async fn setup_game_config(game_name: String, game_dir: String, mod_dir: String) -> Result<(), String> {
	tokio::task::spawn_blocking(move || {
		log::info!(
		"setup_game_config: game_name: {}, game_dir: {}, mod_dir: {}",
		game_name,
		game_dir,
		mod_dir
		);
		let game_dir = std::path::PathBuf::from(game_dir);
		let mod_dir = std::path::PathBuf::from(mod_dir);
		let game_config = config::GameConfig::new(game_name.clone(), game_dir, mod_dir);
		game_config.validate()?;
		GAME_CONFIG.insert(game_name, game_config);
		Ok(())
	})
		.await
		.map_err(|e| format!("Error setting up game config: {}", e))?
}

#[tauri::command(rename_all = "snake_case")]
async fn read_game_config(game_name: &str) -> Result<serde_json::Value, String> {
	log::info!("read_game_config: game_name: {}", game_name);
	let game_config = GAME_CONFIG
		.get(game_name)
		.ok_or_else(|| format!("Game config {} not found", game_name))?;
	tokio::task::spawn_blocking(move || {
		let _ = game_config.value().validate();
		serde_json::to_value(game_config.value()).map_err(|e| e.to_string())
	})
		.await
		.map_err(|e| format!("Error reading game config: {}", e))?
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
	let mut app = tauri::Builder::default()
		.plugin(tauri_plugin_log::Builder::new().build())
		.plugin(tauri_plugin_shell::init())
		.plugin(tauri_plugin_dialog::init())
		.plugin(
			tauri_plugin_log::Builder::new().build(),
		)
		.invoke_handler(
			tauri::generate_handler![
				setup_game_config,
				read_game_config
			])
		.setup(|app| {
			load_game_config();
			Ok(())
		})
		.build(tauri::generate_context!())
		.expect("error while building tauri application");

	app.run(move |_app_handle, _event| match &_event {
		RunEvent::ExitRequested { .. } => {
			save_game_config();
		}
		_ => {}
	});
}

#[cfg(test)]
mod tests {
	use super::*;
	use log::LevelFilter;

	#[test]
	fn test_load_config() {
		env_logger::builder().filter_level(LevelFilter::Info).init();
		load_game_config();
	}

	#[test]
	fn test_save_config() {
		env_logger::builder().filter_level(LevelFilter::Info).init();
		let _ = setup_game_config("test".to_string(), ".".to_string(), "src".to_string());
		save_game_config();
	}

	#[test]
	fn test_setup_game_config() {
		let _ = setup_game_config("test".to_string(), ".".to_string(), "src".to_string());
		let binding = GAME_CONFIG.get("test").unwrap();
		let game_config = binding.value();
		println!("{:?}", game_config);
	}
}
