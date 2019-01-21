// wengwengweng

//! Simple ECS

use std::collections::HashMap;
use std::collections::HashSet;
use std::any::TypeId;
use std::any::Any;

use crate::*;

const MODS: [&str; 17] = [

	"super",
	"cool",
	"awesome",
	"handsome",
	"badass",
	"hotdog",
	"fallen",
	"haunted",
	"king",
	"doomed",
	"forbidden",
	"unstoppable",
	"flaming",
	"unholy",
	"infernal",
	"dwarven",
	"cursed",

];

pub trait Comp: Any {}

pub trait System {

	fn accept(&self);
	fn run(&self);

}

type CompFilter = HashSet<TypeId>;

#[macro_export]
macro_rules! compfilter {

	($($comp:ty),*) => {

		{

			use std::collections::HashSet;
			use std::any::TypeId;

			let mut set = HashSet::new();

			$(
				set.insert(TypeId::of::<$comp>());
			),*

			set

		}

	}

}

#[derive(Default)]
pub struct Scene {

	count: usize,
	entities: HashMap<String, Entity>,

}

impl Scene {

	pub fn new() -> Self {
		return Self::default();
	}

	pub fn add(&mut self, e: Entity) {

		let id = format!("{}{}", MODS[rand!(MODS.len()) as usize], self.count);
		let mut e = self.entities.insert(id.clone(), e).expect("failed to add entity");

		self.count += rand!(9) as usize;
		e.id = Some(id);

	}

	pub fn remove(&mut self, e: Entity) {

		let mut e = self.entities
			.remove(&e.id.expect("invalid entity"))
			.expect("failed to remove entity");

		e.id = None;

	}

	pub fn get(&self, filter: &CompFilter) -> Vec<&Entity> {

		let mut list = Vec::new();

		for (_, e) in &self.entities {
			if e.has_all(filter) {
				list.push(e);
			}
		}

		return list;

	}

}

pub struct Entity {

	comps: HashMap<TypeId, Box<Any>>,
	id: Option<String>,

}

impl Entity {

	pub fn new() -> Self {
		return Self {
			comps: HashMap::new(),
			id: None,
		};
	}

	pub fn with<C: Comp>(&mut self, comp: C) {

		if self.has::<C>() {
			panic!("already have comp");
		} else {
			self.comps.insert(TypeId::of::<C>(), Box::new(comp));
		}

	}

	pub fn has<C: Comp>(&self) -> bool {
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

	pub fn get<C: Comp>(&self) -> &C {

		return self.comps
			.get(&TypeId::of::<C>())
			.map(|c| c.downcast_ref().unwrap())
			.expect("failed to get comp");

	}

	pub fn get_mut<C: Comp>(&mut self) -> &C {

		return self.comps
			.get_mut(&TypeId::of::<C>())
			.map(|c| c.downcast_ref().unwrap())
			.expect("failed to get comp");

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

