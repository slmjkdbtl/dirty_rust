// wengwengweng

macro_rules! wrap {
	($name:ident) => {
		pub fn $name(s: &str) -> String {
			return console::style(s).$name().to_string();
		}
	}
}

wrap!(black);
wrap!(red);
wrap!(green);
wrap!(yellow);
wrap!(blue);
wrap!(magenta);
wrap!(cyan);
wrap!(white);
wrap!(bold);
wrap!(italic);

