use super::module::Module;
use derive_builder::Builder;
use std::rc::Rc;
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
