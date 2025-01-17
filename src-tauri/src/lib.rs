mod config;
mod file;
use dashmap::DashMap;
use std::sync::LazyLock;
use log::{info, debug, error};
use tauri::Manager;
use serde_yaml;

static GAME_CONFIG: LazyLock<DashMap<String, config::GameConfig>> =
	LazyLock::new(|| DashMap::new());

static CONFIG: LazyLock<config::Config> = LazyLock::new(|| {
	config::Config::default()
});


// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
	format!("Hello, {}! You've been greeted from Rust!", name)
}

fn load_game_config() {
	if CONFIG.config_save_path.exists() {
		info!("Loading config from {:?}", CONFIG.config_save_path);

		let content = match std::fs::read(&CONFIG.config_save_path) {
			Ok(c) => c,
			Err(e) => {
				eprintln!("Error reading file {:?}: {}", CONFIG.config_save_path, e);
				return;
			}
		};
		let game_configs: Vec<config::GameConfig> = match serde_yaml::from_slice(&content) {
			Ok(gc) => gc,
			Err(e) => {
				eprintln!("Error parsing config file {:?}: {}", CONFIG.config_save_path, e);
				return;
			}
		};
		for game_config in game_configs {
			GAME_CONFIG.insert(game_config.game_name.clone(), game_config);
		}
	} else {
		info!("Config file {:?} not found, using default config", CONFIG.config_save_path);
	}
}

fn save_game_config() {
	let game_configs: Vec<config::GameConfig> = GAME_CONFIG
			.iter()
			.map(|gc| gc.value().clone())
			.collect();
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
		Err(e) => error!("Error saving config to {:?}: {}", CONFIG.config_save_path, e),
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
	let app = tauri::Builder::default()
			.setup(|app| {
				load_game_config();

				let window = app.get_webview_window("main").unwrap();

				if let Some(window) = app.get_webview_window("main") {
					// 使用 on_window_event 监听事件
					window.clone().on_window_event(move |event| {
						if let tauri::WindowEvent::CloseRequested { api, .. } = event {
							// 阻止窗口直接关闭
							api.prevent_close();

							// 保存配置
							save_game_config();

							// 现在可以手动关闭窗口
							window.close().unwrap();
						}
					});
				}

				Ok(())
			})
			.plugin(tauri_plugin_shell::init())
			.invoke_handler(tauri::generate_handler![greet, setup_game_config])
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
	fn test_load_config() {
		env_logger::builder().filter_level(LevelFilter::Info).init();
		load_game_config();
	}

	#[test]
	fn test_save_config() {
		env_logger::builder().filter_level(LevelFilter::Info).init();
		setup_game_config("test", ".", "src");
		save_game_config();
	}


	#[test]
	fn test_setup_game_config() {
		setup_game_config("test", ".", "src");
		let binding = GAME_CONFIG.get("test").unwrap();
		let game_config = binding.value();
		println!("{:?}", game_config);
	}
}
