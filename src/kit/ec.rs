// wengwengweng

//! Simple Entity and Component

use std::collections::HashMap;
use std::collections::HashSet;
use std::any::TypeId;
use std::any::Any;

pub type CompFilter = HashSet<TypeId>;

#[macro_export]
macro_rules! comps {

	() => {
		std::collections::HashSet::new()
	};

	($($comp:ty),*) => {

		{

			let mut set = std::collections::HashSet::new();

			$(
				set.insert(std::any::TypeId::of::<$comp>());
			)*

			set

		}

	};

}

#[macro_export]
macro_rules! entity {
	($($comp:expr),*) => {
		{
			let mut e = Entity::new();
			$(
				e.with($comp);
			)*
			e
		}
	}
}

#[derive(Default)]
pub struct Entity {
	comps: HashMap<TypeId, Box<Any>>,
}

impl Entity {

	pub fn new() -> Self {
		return Self::default();
	}

	pub fn with<C: Any>(&mut self, comp: C) {

		if self.has::<C>() {
			panic!("cannot have duplicate comps");
		} else {
			self.comps.insert(TypeId::of::<C>(), Box::new(comp));
		}

	}

	pub fn has<C: Any>(&self) -> bool {
		return self.comps.contains_key(&TypeId::of::<C>());
	}

	pub fn has_all(&self, comps: &CompFilter) -> bool {

		for f in comps {
			if !self.comps.contains_key(&f) {
				return false;
			}
		}

		return true;

	}

	pub fn borrow<C: Any>(&self) -> &C {

		return self.comps
			.get(&TypeId::of::<C>())
			.map(|c| c.downcast_ref().unwrap())
			.expect("failed to get comp");

	}

	pub fn borrow_mut<C: Any>(&mut self) -> &C {

		return self.comps
			.get_mut(&TypeId::of::<C>())
			.map(|c| c.downcast_mut().unwrap())
			.expect("failed to get comp");

	}

	pub fn get<C: Any + Clone>(&self) -> C {

		return self.comps
			.get(&TypeId::of::<C>())
			.map(|c| c.downcast_ref().unwrap())
			.map(Clone::clone)
			.expect("failed to get comp");

	}

	pub fn set<C: Any>(&mut self, comp: C) {

		self.comps
			.insert(TypeId::of::<C>(), Box::new(comp));

	}

}

