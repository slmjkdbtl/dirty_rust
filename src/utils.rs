// wengwengweng

//! Misc Helpers

pub use once_cell;

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

#[macro_export]
macro_rules! log {
	($t:tt) => {
		log!("{}", $t)
	};
	($($t:tt)*) => {
		#[cfg(web)]
		web_sys::console::log_1(&format_args!($($t)*).to_string().into());
		#[cfg(not(web))]
		println!($($t)*,);
	};
}

#[macro_export]
macro_rules! hmap {
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

#[macro_export]
macro_rules! hset {
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

#[macro_export]
macro_rules! bmap {
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

#[macro_export]
macro_rules! bset {
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

#[macro_export]
macro_rules! vecd {
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

#[macro_export]
macro_rules! llist {
	($($item:expr),*$(,)?) => {
		{
			let mut _tmp = std::collections::LinkedList::new();
			if false {
				_tmp
			} else {
				$(_tmp.push_back($item);)*
				_tmp
			}
		}
	}
}

#[test]
fn collection_macros() {

	hmap![
		"123" => 123,
		"456" => 456,
	];

	bmap![
		"123" => 123,
		"456" => 456,
	];

	hset![
		123,
		456,
	];

	bset![
		123,
		456,
	];

	vecd![
		123,
		456,
	];

	llist![
		123,
		456,
	];

}

