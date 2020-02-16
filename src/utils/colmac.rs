// wengwengweng

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

