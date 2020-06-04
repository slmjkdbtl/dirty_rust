// wengwengweng

//! General Utilities

use std::panic;

#[allow(unused_macros)]
macro_rules! export {
	($name:ident) => {
		mod $name;
		pub use $name::*;
	}
}

#[allow(unused_macros)]
macro_rules! import {
	($name:ident) => {
		mod $name;
		#[allow(unused)]
		use $name::*;
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

/// cross-platform console output
#[macro_export]
macro_rules! log {
	($($t:tt)*) => {
		#[cfg(web)]
		web_sys::console::log_1(&format_args!($($t)*).to_string().into());
		#[cfg(not(web))]
		println!($($t)*,);
	};
}

/// cross-platform console error output
#[macro_export]
macro_rules! elog {
	($($t:tt)*) => {
		#[cfg(web)]
		web_sys::console::error_1(&format_args!($($t)*).to_string().into());
		#[cfg(not(web))]
		eprintln!($($t)*,);
	};
}

/// create a HashMap
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

/// create a HashSet
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

/// create a BTreeMap
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

/// create a BTreeSet
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

/// create a VecDeque
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

/// create a LinkedList
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

#[macro_export]
macro_rules! hash {

	($s:expr) => {{

		use std::hash::Hash;
		use std::hash::Hasher;

		let mut _hasher = std::collections::hash_map::DefaultHasher::new();

		$s.hash(&mut _hasher);
		_hasher.finish()

	}};

	($($s:expr),*) => {{
		let mut _hash: u128 = 0;
		$(_hash += $crate::hash!($s) as u128;)*
		$crate::hash!(_hash)
	}};

}

/// simple wrapper for panic hook
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

/// use a default cross-platform panic hook
pub fn use_dirty_panic() {

	set_panic(|msg, loc| {

		use crate::term::style as s;

		let loc = loc
			.map(|loc| format!("{}:{}:{}", loc.file(), loc.line(), loc.column()))
			.unwrap_or("".to_string());

		elog!("{} {}:\n{}", s("PANIC").bold().red(), loc, msg.unwrap_or(""));

	});

}

