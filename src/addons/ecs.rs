// wengwengweng

//! Simple ECS

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BTreeMap;
use std::any::TypeId;
use std::any::Any;
use std::fmt;
use std::cmp::Ordering;

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
pub type CompFilter = HashSet<TypeId>;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Id(&'static str, usize);

impl Id {
	pub fn new(id: usize) -> Self {
		return Self(MODS[rand!(MODS.len()) as usize], id);
	}
}

impl Ord for Id {
	fn cmp(&self, other: &Id) -> Ordering {
		return self.1.cmp(&other.1);
	}
}

impl PartialOrd for Id {
	fn partial_cmp(&self, other: &Id) -> Option<Ordering> {
		return Some(self.1.cmp(&other.1));
	}
}

impl fmt::Debug for Id {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		return write!(f, "{}{}", self.0, self.1);
	}
}

impl fmt::Display for Id {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		return write!(f, "{:?}", self.0);
	}
}

#[macro_export]
macro_rules! comp_filter {

	($($comp:ty),*) => {

		{

			let mut set = std::collections::HashSet::new();

			$(
				set.insert(std::any::TypeId::of::<$comp>());
			)*

			set

		}

	}

}

#[derive(Default)]
pub struct Scene {

	count: usize,
	entities: BTreeMap<Id, Entity>,

}

impl Scene {

	pub fn new() -> Self {
		return Self::default();
	}

	pub fn add(&mut self, mut e: Entity) -> Id {

		let id = Id::new(self.count);

		e.id = Some(id);
		self.entities.insert(id, e);
		self.count += rand!(2, 9) as usize;

		return id;

	}

	pub fn get(&self, id: Id) -> Option<&Entity> {
		return self.entities.get(&id);
	}

	pub fn get_mut(&mut self, id: Id) -> Option<&mut Entity> {
		return self.entities.get_mut(&id);
	}

	pub fn filter(&self, filter: CompFilter) -> Vec<Id> {

		let mut list = Vec::new();

		for (id, e) in &self.entities {
			if e.has_all(&filter) {
				list.push(*id);
			}
		}

		return list;

	}

	pub fn remove(&mut self, e: Id) {

		self.entities.remove(&e);

	}

	pub fn size(&self) -> usize {
		return self.entities.len();
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

pub struct Entity {

	comps: HashMap<TypeId, Box<Any>>,
	pub id: Option<Id>,

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
			panic!("cannot have duplicate comps");
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

	pub fn get<C: Comp + Clone>(&self) -> C {

		return self.comps
			.get(&TypeId::of::<C>())
			.map(|c| c.downcast_ref().unwrap())
			.map(Clone::clone)
			.expect("failed to get comp");

	}

	pub fn set<C: Comp>(&mut self, comp: C) {

		self.comps
			.insert(TypeId::of::<C>(), Box::new(comp));

	}

}

pub fn scene() -> Scene {
	return Scene::new();
}

#[macro_export]
macro_rules! comp {

	($name:ident { $($member:ident: $type:ty),*$(,)? }) => {

		#[derive(Clone)]
		pub struct $name {
			$(
				pub $member: $type
			),*
		}

		impl Comp for $name {}

	};

	($name:ident) => {

		#[derive(Clone)]
		pub struct $name;
		impl Comp for $name {}

	};

}

