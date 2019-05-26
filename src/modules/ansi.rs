// wengwengweng

use colored::*;

macro_rules! wrap {
	($name:ident) => {
		pub fn $name(s: &str) -> String {
			return Colorize::$name(s).to_string();
		}
	}
}

wrap!(black);
wrap!(red);
wrap!(green);
wrap!(yellow);
wrap!(blue);
wrap!(magenta);
wrap!(purple);
wrap!(cyan);
wrap!(white);
wrap!(bold);
wrap!(italic);
wrap!(normal);

