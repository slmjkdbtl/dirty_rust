// wengwengweng

#[allow(unused_macros)]
macro_rules! export {
	($name:ident) => {
		mod $name;
		pub use $name::*;
	}
}

#[allow(unused_macros)]
macro_rules! hashmap {
	($($key:expr => $val:expr),*$(,)?) => {
		{
			let mut _tmp = std::collections::HashMap::new();
			$(_tmp.insert($key, $val);)*
			_tmp
		}
	}
}

#[allow(unused_macros)]
macro_rules! hashset {
	($($item:expr),*$(,)?) => {
		{
			let mut _tmp = std::collections::HashSet::new();
			$(_tmp.insert($item);)*
			_tmp
		}
	};
}

