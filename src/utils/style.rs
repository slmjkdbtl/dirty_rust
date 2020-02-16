// wengwengweng

#[derive(Clone)]
pub struct StyledOutput {
	text: String,
}

fn ansi_wrap(s: &str, code: &str) -> String {
	return format!("\x1b[{}m{}\x1b[0m", code, s);
}

macro_rules! ansi {
	({$($name:ident => $code:expr),*$(,)?}) => {
		impl StyledOutput {
			$(
				pub fn $name(mut self) -> Self {
					self.text = ansi_wrap(&self.text, $code);
					return self;
				}
			)*
		}
	}
}

ansi!({
	black => "30",
	red => "31",
	green => "32",
	yellow => "33",
	blue => "34",
	magenta => "35",
	cyan => "36",
	white => "37",
});

ansi!({
	bg_black => "40",
	bg_red => "41",
	bg_green => "42",
	bg_yellow => "44",
	bg_blue => "44",
	bg_magenta => "45",
	bg_cyan => "46",
	bg_white => "47",
});

ansi!({
	bold => "1",
	dim => "2",
	italic => "3",
	underline => "4",
	blink => "5",
});

impl StyledOutput {
	pub fn truec(mut self, c: crate::math::Color) -> Self {
		let c = c.as_u8();
		self.text = ansi_wrap(&self.text, &format!("38;2;{};{};{}", c[0], c[1], c[2]));
		return self;
	}
	pub fn bg_truec(mut self, c: crate::math::Color) -> Self {
		let c = c.as_u8();
		self.text = ansi_wrap(&self.text, &format!("48;2;{};{};{}", c[0], c[1], c[2]));
		return self;
	}
}

impl std::fmt::Display for StyledOutput {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
		return write!(f, "{}", self.text);
	}
}

impl std::fmt::Debug for StyledOutput {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
		return write!(f, "{}", self);
	}
}

pub fn style(s: &str) -> StyledOutput {
	return StyledOutput {
		text: String::from(s),
	};
}

