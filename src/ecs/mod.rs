// wengwengweng

//! Simple ECS

pub use ecs_derive as derive;

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
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

pub trait System {

	fn filter(&self) -> Filter {
		return filter![];
	}

	fn update(&mut self, pool: &mut Pool) {}
	fn each(&mut self, e: &mut Entity) {}

}

pub trait Comp: Any {}
pub type Filter = HashSet<TypeId>;
pub type EntitySet = BTreeSet<Id>;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
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

impl fmt::Display for Id {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		return write!(f, "{}{}", self.0, self.1);
	}
}

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
	count: &'a mut usize,

}

impl<'a> Pool<'a> {

	pub fn add(&mut self,e: Entity) -> Id {

		let id = Id::new(*self.count);

		self.entities.insert(id, e);
		*self.count += rand!(2, 7) as usize;

		return id;

	}

	pub fn remove(&mut self, e: Id) {
		self.entities.remove(&e);
	}

	pub fn filter(&self, filter: &Filter) -> EntitySet {

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

}

struct SystemData {

	system: Box<System>,
	filter: Filter,

}

#[derive(Default)]
pub struct World {

	count: usize,
	entities: BTreeMap<Id, Entity>,
	systems: Vec<SystemData>,

}

impl World {

	pub fn new() -> Self {
		return Self::default();
	}

	pub fn add(&mut self, e: Entity) -> Id {

		let id = Id::new(self.count);

		self.entities.insert(id, e);
		self.count += rand!(2, 7) as usize;

		return id;

	}

	pub fn run<S: System + 'static>(&mut self, system: S) {

		self.systems.push(SystemData {

			filter: system.filter(),
			system: Box::new(system),

		});

	}

	pub fn update(&mut self) {

		let mut pool = Pool {
			count: &mut self.count,
			entities: &mut self.entities,
		};

		for s in &mut self.systems {

			for id in &pool.filter(&s.filter) {
				s.system.each(pool.get_mut(*id).unwrap());
			}

			s.system.update(&mut pool);

		}

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
}

impl Entity {

	pub fn new() -> Self {
		return Self {
			comps: HashMap::new(),
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

	pub fn has_all(&self, comps: &Filter) -> bool {

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

