// wengwengweng

//! Simple ECS

use std::collections::HashMap;
use std::any::TypeId;
use std::any::Any;

pub trait Comp {}
pub trait System {}
pub struct Scene {}

pub struct Entity {
	comps: HashMap<TypeId, Box<Comp>>,
}

impl Entity {

	pub fn new() -> Self {
		return Self {
			comps: HashMap::new(),
		};
	}

	pub fn with<C: Any + Comp>(&mut self, comp: C) {

		if self.has::<C>() {
			panic!("already have comp");
		} else {
			self.comps.insert(TypeId::of::<C>(), Box::new(comp));
		}

	}

	pub fn has<C: Any + Comp>(&self) -> bool {
		return self.comps.contains_key(&TypeId::of::<C>());
	}

}

pub fn entity() -> Entity {
	return Entity::new();
}

#[macro_export]
macro_rules! comp {

	($name:ident { $($member:ident: $type:ident ($default:expr)),+$(,)? }) => {

		#[derive(Debug)]
		pub struct $name {
			$(
				pub $member: $type
			),*
		}

		impl $name {
			pub fn new() -> Self {
				return Self::default();
			}
		}

		impl Comp for $name {}

		impl Default for $name {
			fn default() -> $name {
				return $name {
					$(
						$member: $default
					),*
				};
			}
		}

	};

}

