use std::rc::Rc;
use super::module::Module;
use derive_builder::Builder;
#[derive(Builder, Debug, Clone)]
pub struct Dependency {
	name: String,
	module_ref: Option<Rc<Module>>,
}


impl Dependency {
	pub fn new(name: String, module_ref: Option<Rc<Module>>) -> Dependency {
		Dependency {
			name,
			module_ref,
		}
	}
}
