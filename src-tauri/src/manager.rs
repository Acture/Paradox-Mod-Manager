use crate::module::component::Component;
use crate::module::dependency::DependencyBuilder;
use crate::module::meta::Meta;
use crate::module::module::Module;
use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;
struct ModuleManager {
	modules: HashMap<String, Rc<Module>>,
}

impl ModuleManager {
	pub fn new() -> ModuleManager {
		ModuleManager {
			modules: HashMap::new(),
		}
	}

	pub fn add_module(&mut self, module: Module) {
		self.modules
			.insert(module.meta.name.clone(), Rc::new(module));
	}

	pub fn find_module(&self, name: &str) -> Option<Rc<Module>> {
		self.modules.get(name).cloned()
	}

	pub fn create_from_mod_file<P: AsRef<Path>>(
		&self,
		file_path: P,
	) -> Result<Module, Box<dyn std::error::Error>> {
		let mut is_valid = true;
		let meta = Meta::create_from_mod_file(file_path)?;

		let components = match Component::get_components_from_dir(&meta.local_path) {
			Ok(components) => components,
			Err(e) => {
				eprintln!("Error getting components from directory: {}", e);
				is_valid = false;
				Vec::new()
			}
		};

		let dependencies = meta
			.dependencies
			.iter()
			.map(|dep| {
				DependencyBuilder::default()
					.name(dep.clone())
					.module_ref(match self.find_module(dep) {
						Some(module) => Some(module),
						None => None,
					})
					.build()
					.unwrap()
			})
			.collect();

		Ok(Module::new(meta, components, dependencies, is_valid))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::utility::get_mods_metafiles;
	use std::path::Path;

	#[test]
	fn test_module_manager() {
		let mut manager = ModuleManager::new();
		let mod_dir = Path::new("D:/actur/Documents/Paradox Interactive/Europa Universalis IV/mod");
		let files = get_mods_metafiles(mod_dir);
		for file in files {
			let module = manager.create_from_mod_file(file).unwrap();
			manager.add_module(module);
			break;
		}
		println!("{:?}", manager.modules);
	}
}
