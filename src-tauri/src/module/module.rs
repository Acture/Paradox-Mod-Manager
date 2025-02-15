use std::path::{Path, PathBuf};
use super::component::Component;
use super::dependency::Dependency;
use super::meta::{Meta, MetaBuilder};
use derive_builder::Builder;
use crate::lang::parser::{parse_content};

#[derive(Builder, Debug)]
pub struct Module {
	pub meta: Meta,
	pub components: Vec<Component>,
	pub dependencies: Vec<Dependency>,
	pub is_valid: bool,
}

impl Module {
	pub fn new(meta: Meta, components: Vec<Component>, dependencies: Vec<Dependency>, is_valid: bool) -> Module {
		Module {
			meta,
			components,
			dependencies,
			is_valid,
		}
	}
}
