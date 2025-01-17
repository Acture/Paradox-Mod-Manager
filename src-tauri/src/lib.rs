mod config;
mod file;
use dashmap::DashMap;
use std::sync::LazyLock;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
	format!("Hello, {}! You've been greeted from Rust!", name)
}

static GAME_CONFIG: LazyLock<DashMap<String, config::GameConfig>> =
	LazyLock::new(|| DashMap::new());

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
	use super::*;

	#[test]
	fn test_greet() {
		assert_eq!(
			greet("world"),
			"Hello, world! You've been greeted from Rust!"
		);
	}

	#[test]
	fn test_setup_game_config() {
		setup_game_config("test", ".", "src");
		let binding = GAME_CONFIG.get("test").unwrap();
		let game_config = binding.value();
		println!("{:?}", game_config);
	}
}
