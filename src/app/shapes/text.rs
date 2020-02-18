// wengwengweng

use super::*;

#[derive(Clone)]
pub struct FormattedChar {
	ch: char,
	pos: Vec2,
	tex: gfx::Texture,
	quad: Quad,
	width: f32,
}

#[derive(Clone)]
pub struct FormattedLine {
	chars: Vec<FormattedChar>,
	width: f32,
}

impl FormattedLine {
	fn new() -> Self {
		return Self {
			chars: vec![],
			width: 0.0,
		};
	}
}

#[derive(Clone)]
pub struct FormattedText {
	chars: Vec<FormattedChar>,
	scale: f32,
	width: f32,
	height: f32,
	color: Color,
	italic: bool,
	bold: bool,
}

impl FormattedText {

	pub fn width(&self) -> f32 {
		return self.width;
	}
	pub fn height(&self) -> f32 {
		return self.height;
	}

	pub fn cursor_pos(&self, i: usize) -> Option<Vec2> {
		return self.chars.get(i).map(|ch| {
			return ch.pos;
		});
	}

	// TODO
	pub fn pos_cursor(&self, pos: Vec2) -> Option<usize> {
		return None;
	}

}

impl gfx::Drawable for FormattedText {

	fn draw(&self, ctx: &mut app::Ctx) -> Result<()> {

		let italic = if self.italic {
			0.3
		} else {
			0.0
		};

		for fch in &self.chars {
			ctx.draw_t(mat4!()
				.t2(fch.pos)
				.skx(italic)
				.tx(italic * fch.width)
				.s2(vec2!(self.scale))
			, &sprite(&fch.tex)
				.offset(vec2!(-1, 1))
				.quad(fch.quad)
				.color(self.color)
			)?;
		}

		return Ok(());
	}

}

#[derive(Clone)]
pub struct TextWrap {
	pub width: f32,
	pub break_word: bool,
	pub hyphonate: bool,
}

#[derive(Clone)]
pub struct FormatConf {
	pub align: gfx::Origin,
	pub wrap: Option<TextWrap>,
	pub size: Option<f32>,
	pub line_spacing: f32,
	pub char_spacing: f32,
	pub color: Color,
	pub italic: bool,
	pub bold: bool,
}

impl Default for FormatConf {
	fn default() -> Self {
		return Self {
			align: gfx::Origin::TopLeft,
			wrap: None,
			size: None,
			line_spacing: 0.0,
			char_spacing: 0.0,
			color: rgba!(1),
			italic: false,
			bold: false,
		};
	}
}

fn format(text: &str, font: &dyn gfx::Font, conf: &FormatConf) -> FormattedText {

	let mut lines = vec![];
	let mut cur_line = FormattedLine::new();
	let scale = conf.size.map(|s| s / font.height()).unwrap_or(1.0);
	let gh = font.height() * scale + conf.line_spacing;
	let mut w = 0.0;
	let mut break_pt: Option<FormattedLine> = None;

	for ch in text.chars() {

		if ch == '\n' {
			lines.push(std::mem::replace(&mut cur_line, FormattedLine::new()));
		} else {

			if let Some((tex, quad)) = font.get(ch) {

				let gw = tex.width() as f32 * quad.w * scale + conf.char_spacing;

				if let Some(wrap) = &conf.wrap {
					if cur_line.width + gw > wrap.width {
						if let Some(line) = break_pt.take() {
							cur_line.width -= line.width;
							cur_line.chars.drain(0..line.chars.len());
							lines.push(line);
						} else {
							lines.push(std::mem::replace(&mut cur_line, FormattedLine::new()));
						}
					}
				}

				cur_line.chars.push(FormattedChar {
					ch: ch,
					pos: vec2!(),
					tex: tex.clone(),
					quad: quad,
					width: gw - conf.char_spacing,
				});

				cur_line.width += gw;
				w = f32::max(cur_line.width, w);

			}

			if ch == ' ' {
				if let Some(wrap) = &conf.wrap {
					if !wrap.break_word {
						break_pt = Some(cur_line.clone());
					}
				}
			}

		}

	}

	lines.push(cur_line);

	let h = lines.len() as f32 * gh;
	let offset_pt = conf.align.as_pt() * 0.5 + vec2!(0.5, -0.5);
	let offset = -offset_pt * vec2!(w, h);

	let mut fchars = vec![];
	let mut x = 0.0;

	for (i, line) in lines
		.into_iter()
		.enumerate() {

		let ox = (w - line.width) * offset_pt.x;
		let y = i as f32 * -gh;

		for ch in line.chars {

			let gw = ch.tex.width() as f32 * ch.quad.w * scale + conf.char_spacing;

			fchars.push(FormattedChar {
				pos: vec2!(x + ox, y) + offset,
				..ch
			});

			x += gw;

		}

		x = 0.0;

	}

	return FormattedText {
		chars: fchars,
		scale: scale,
		width: w,
		height: h,
		color: conf.color,
		italic: conf.italic,
		bold: conf.bold,
	};

}

#[derive(Clone)]
pub struct Text<'a> {
	content: &'a str,
	font: Option<&'a dyn gfx::Font>,
	conf: FormatConf,
}

impl<'a> Text<'a> {
	pub fn new(s: &'a str) -> Self {
		return Self {
			content: s,
			font: None,
			conf: FormatConf::default(),
		};
	}
	pub fn font(mut self, f: &'a dyn gfx::Font) -> Self {
		self.font = Some(f);
		return self;
	}
	pub fn color(mut self, color: Color) -> Self {
		self.conf.color = color;
		return self;
	}
	pub fn opacity(mut self, a: f32) -> Self {
		self.conf.color.a = a;
		return self;
	}
	pub fn align(mut self, o: gfx::Origin) -> Self {
		self.conf.align = o;
		return self;
	}
	pub fn size(mut self, s: f32) -> Self {
		self.conf.size = Some(s);
		return self;
	}
	pub fn wrap(mut self, wrap: TextWrap) -> Self {
		self.conf.wrap = Some(wrap);
		return self;
	}
	pub fn line_spacing(mut self, h: f32) -> Self {
		self.conf.line_spacing = h;
		return self;
	}
	pub fn italic(mut self, b: bool) -> Self {
		self.conf.italic = b;
		return self;
	}
	pub fn bold(mut self, b: bool) -> Self {
		self.conf.bold = b;
		return self;
	}
}

pub fn text<'a>(s: &'a str) -> Text<'a> {
	return Text::new(s);
}

impl<'a> Text<'a> {
	pub fn format(&self, ctx: &app::Ctx) -> FormattedText {
		return format(self.content, self.font.unwrap_or(ctx.default_font()), &self.conf);
	}
}

impl<'a> gfx::Drawable for Text<'a> {

	fn draw(&self, ctx: &mut app::Ctx) -> Result<()> {
		return ctx.draw(&self.format(ctx));
	}

}

