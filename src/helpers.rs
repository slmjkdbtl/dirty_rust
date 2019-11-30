// wengwengweng

#[allow(unused_macros)]
macro_rules! export {
	($name:ident) => {
		mod $name;
		pub use $name::*;
	}
}

#[allow(unused_macros)]
macro_rules! mexport {
	($name:ident) => {
		#[macro_use]
		mod $name;
		pub use $name::*;
	}
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! hashmap {
	($($key:expr => $val:expr),*$(,)?) => {
		{
			let mut _tmp = std::collections::HashMap::new();
			if false {
				_tmp
			} else {
				$(_tmp.insert($key, $val);)*
				_tmp
			}
		}
	}
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! hashset {
	($($item:expr),*$(,)?) => {
		{
			let mut _tmp = std::collections::HashSet::new();
			if false {
				_tmp
			} else {
				$(_tmp.insert($item);)*
				_tmp
			}
		}
	};
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! btreemap {
	($($key:expr => $val:expr),*$(,)?) => {
		{
			let mut _tmp = std::collections::BTreeMap::new();
			if false {
				_tmp
			} else {
				$(_tmp.insert($key, $val);)*
				_tmp
			}
		}
	}
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! btreeset {
	($($item:expr),*$(,)?) => {
		{
			let mut _tmp = std::collections::BTreeSet::new();
			if false {
				_tmp
			} else {
				$(_tmp.insert($item);)*
				_tmp
			}
		}
	}
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! vecdeque {
	($($item:expr),*$(,)?) => {
		{
			let mut _tmp = std::collections::VecDeque::new();
			if false {
				_tmp
			} else {
				$(_tmp.push_back($item);)*
				_tmp
			}
		}
	}
}

use std::panic;

pub fn set_panic<F: 'static + Fn(Option<&str>, Option<&panic::Location>) + Send + Sync>(f: F) {

	panic::set_hook(Box::new(move |info: &panic::PanicInfo| {

		let msg: Option<&str> = if let Some(s) = info.payload().downcast_ref::<&str>() {
			Some(*s)
		} else if let Some(s) = info.payload().downcast_ref::<String>() {
			Some(&s)
		} else {
			None
		};

		f(msg, info.location());

	}));

}

#[test]
fn collection_macros() {

	hashmap![
		"123" => 123,
		"456" => 456,
	];

	btreemap![
		"123" => 123,
		"456" => 456,
	];

	hashset![
		123,
		456,
	];

	btreeset![
		123,
		456,
	];

	vecdeque![
		123,
		456,
	];

}

