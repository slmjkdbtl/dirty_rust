// wengwengweng

//! Simple Ecs

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::any::TypeId;
use std::any::Any;

use crate::utils::id::*;

pub use crate::utils::id::Id;

pub type Filter = HashSet<TypeId>;
type EntitySet = BTreeSet<Id>;

#[macro_export]
macro_rules! filter {

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

pub struct Pool<'a> {

	entities: &'a mut BTreeMap<Id, Entity>,
	id_generator: &'a mut IdGenerator,
	resources: &'a mut HashMap<TypeId, Box<Any>>,

}

impl<'a> Pool<'a> {

	pub fn push(&mut self,e: Entity) -> Id {

		let id = self.id_generator.get();

		self.entities.insert(id, e);

		return id;

	}

	pub fn remove(&mut self, e: Id) {
		self.entities.remove(&e);
	}

	pub fn pick(&self, filter: &Filter) -> EntitySet {

		let mut list = BTreeSet::new();

		for (id, e) in self.entities.iter() {
			if e.has_all(&filter) {
				list.insert(*id);
			}
		}

		return list;

	}

	pub fn get(&self, id: Id) -> Option<&Entity> {
		return self.entities.get(&id);
	}

	pub fn get_mut(&mut self, id: Id) -> Option<&mut Entity> {
		return self.entities.get_mut(&id);
	}

	pub fn get_res<R: Any>(&self) -> Option<&R> {
		return self.resources
			.get(&TypeId::of::<R>())
			.map(|c| c.downcast_ref().unwrap());
	}

	pub fn get_res_mut<R: Any>(&mut self) -> Option<&mut R> {
		return self.resources
			.get_mut(&TypeId::of::<R>())
			.map(|c| c.downcast_mut().unwrap());
	}

}

#[derive(Default)]
pub struct World {

	id_generator: IdGenerator,
	entities: BTreeMap<Id, Entity>,
	resources: HashMap<TypeId, Box<Any>>,

}

impl World {

	pub fn new() -> Self {
		return Self::default();
	}

	pub fn add(&mut self, e: Entity) -> Id {

		let id = self.id_generator.get();

		self.entities.insert(id, e);

		return id;

	}

	pub fn share<R: Any>(&mut self, resource: R) {
		self.resources.insert(TypeId::of::<R>(), Box::new(resource));
	}

	pub fn get_res<R: Any>(&self) -> Option<&R> {
		return self.resources
			.get(&TypeId::of::<R>())
			.map(|c| c.downcast_ref().unwrap());
	}

	pub fn run<F: Fn(&mut Pool)>(&mut self, system: F) {

		let mut pool = Pool {

			id_generator: &mut self.id_generator,
			entities: &mut self.entities,
			resources: &mut self.resources,

		};

		system(&mut pool);

	}

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

	pub fn has_all(&self, comps: &Filter) -> bool {

		for f in comps {
			if !self.comps.contains_key(&f) {
				return false;
			}
		}

		return true;

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

