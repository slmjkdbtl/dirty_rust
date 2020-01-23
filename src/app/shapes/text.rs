// wengwengweng

use super::*;

#[derive(Clone, Copy, Debug)]
pub struct TextWrap {
	width: f32,
	break_word: bool,
}

#[derive(Clone)]
pub struct Text<'a> {
	content: &'a str,
	font: Option<&'a dyn gfx::Font>,
	color: Color,
	align: Option<gfx::Origin>,
	wrap: Option<TextWrap>,
	size: Option<f32>,
	line_spacing: f32,
	char_spacing: f32,
	italic: Option<f32>,
	bold: Option<f32>,
}

impl<'a> Text<'a> {
	pub fn new(s: &'a str) -> Self {
		return Self {
			content: s,
			font: None,
			align: None,
			color: rgba!(1),
			wrap: None,
			line_spacing: 0.0,
			char_spacing: 0.0,
			size: None,
			italic: None,
			bold: None,
		};
	}
	pub fn font(mut self, f: &'a dyn gfx::Font) -> Self {
		self.font = Some(f);
		return self;
	}
	pub fn color(mut self, color: Color) -> Self {
		self.color = color;
		return self;
	}
	pub fn opacity(mut self, a: f32) -> Self {
		self.color.a = a;
		return self;
	}
	pub fn align(mut self, o: gfx::Origin) -> Self {
		self.align = Some(o);
		return self;
	}
	pub fn size(mut self, s: f32) -> Self {
		self.size = Some(s);
		return self;
	}
	pub fn wrap(mut self, width: f32, break_word: bool) -> Self {
		self.wrap = Some(TextWrap {
			width: width,
			break_word: break_word,
		});
		return self;
	}
	pub fn line_spacing(mut self, h: f32) -> Self {
		self.line_spacing = h;
		return self;
	}
	pub fn italic(mut self, i: f32) -> Self {
		self.italic = Some(i);
		return self;
	}
	pub fn bold(mut self, i: f32) -> Self {
		self.bold = Some(i);
		return self;
	}
}

pub fn text<'a>(s: &'a str) -> Text<'a> {
	return Text::new(s);
}

impl<'a> Text<'a> {

	pub fn render(&self, ctx: &Ctx) -> RenderedText<'a> {

		let font = self.font.unwrap_or(&ctx.default_font);
		let scale = self.size.map(|s| s / font.height()).unwrap_or(1.0);
		let gh = font.height() * scale;
		let mut lines = vec![];
		let mut cx = 0.0;
		let mut cy = gh;
		let mut l = String::new();

		match self.wrap {

			Some(wrap) => {

				let mut last_space: Option<RenderedLine> = None;

				for ch in self.content.chars() {

					if ch == '\n' {

						lines.push(RenderedLine {
							text: mem::replace(&mut l, String::new()),
							width: cx,
						});

						cx = 0.0;
						cy += gh;
						cy += self.line_spacing;

					} else {

						if let Some((tex, quad)) = font.get(ch) {

							let gw = tex.width() as f32 * quad.w * scale;

							if cx + gw > wrap.width {

								if let Some(last_space) = last_space.take() {

									cx = wrap.width - last_space.width;
									cy += gh;
									cy += self.line_spacing;
									l = l.replace(&last_space.text, "");
									l.push(ch);
									lines.push(last_space);

								} else {

									lines.push(RenderedLine {
										text: mem::replace(&mut l, String::new()),
										width: cx,
									});

									cx = 0.0;
									cy += gh;
									cy += self.line_spacing;
									cx += gw;
									l.push(ch);

								}

							} else {

								cx += gw;
								l.push(ch);

							}

						}

						if !wrap.break_word {
							if ch == ' ' {
								last_space = Some(RenderedLine {
									text: l.clone(),
									width: cx,
								});
							}
						}

					}

				}

				lines.push(RenderedLine {
					text: l,
					width: cx,
				});

				return RenderedText {
					width: wrap.width,
					height: cy,
					lines: lines,
					align: self.align.unwrap_or(gfx::Origin::Center),
					font: self.font,
					line_spacing: self.line_spacing,
					char_spacing: self.char_spacing,
					color: self.color,
					size: self.size,
					italic: self.italic,
					bold: self.bold,
				};

			},

			None => {

				for ch in self.content.chars() {

					if ch == '\n' {

						lines.push(RenderedLine {
							text: mem::replace(&mut l, String::new()),
							width: cx,
						});

						cx = 0.0;
						cy += gh;
						cy += self.line_spacing;

					} else {

						if let Some((tex, quad)) = font.get(ch) {

							let gw = tex.width() as f32 * quad.w * scale;

							l.push(ch);
							cx += gw;

						}

					}

				}

				lines.push(RenderedLine {
					text: l,
					width: cx,
				});

				return RenderedText {
					width: cx,
					height: cy,
					lines: lines,
					align: self.align.unwrap_or(gfx::Origin::Center),
					font: self.font,
					line_spacing: self.line_spacing,
					char_spacing: self.char_spacing,
					color: self.color,
					size: self.size,
					italic: self.italic,
					bold: self.bold,
				};

			},

		}

	}

}

#[derive(Clone, Debug)]
pub struct RenderedLine {
	text: String,
	width: f32,
}

#[derive(Clone)]
pub struct RenderedText<'a> {
	width: f32,
	height: f32,
	lines: Vec<RenderedLine>,
	align: gfx::Origin,
	font: Option<&'a dyn gfx::Font>,
	line_spacing: f32,
	char_spacing: f32,
	color: Color,
	size: Option<f32>,
	italic: Option<f32>,
	bold: Option<f32>,
}

impl<'a> RenderedText<'a> {

	pub fn width(&self) -> f32 {
		return self.width;
	}

	pub fn height(&self) -> f32 {
		return self.height;
	}

	// TODO: cursor on last char of line 1 or first char of line 2?
	pub fn cursor_pos(&self, ctx: &Ctx, cpos: i32) -> Option<Vec2> {

		let offset = (self.align.as_pt() + vec2!(1)) * 0.5;
		let offset_pos = -offset * vec2!(self.width, self.height);

		if cpos == 0 {
			return Some(offset_pos);
		}

		let font = self.font.unwrap_or(&ctx.default_font);
		let scale = self.size.map(|s| s / font.height()).unwrap_or(1.0);
		let gh = font.height() * scale + self.line_spacing;
		let mut tl = 0;

		for (y, line) in self.lines.iter().enumerate() {

			tl += line.text.len() as i32;

			if cpos > tl {
				continue;
			} else {

				let mut x = 0.0;
				let ox = (self.width - line.width) * offset.x;
				let ccpos = cpos - tl + line.text.len() as i32 - 1;

				for (i, ch) in line.text.chars().enumerate() {

					if let Some((tex, quad)) = font.get(ch) {

						let gw = tex.width() as f32 * quad.w * scale;
						x += gw;

						if i as i32 == ccpos {
							return Some(offset_pos + vec2!(x + ox, y as f32 * gh));
						}

					}
				}

			}

		}

		return None;

	}

	// TODO
	pub fn pos_cursor(&self, ctx: Ctx, pos: Vec2) -> Option<i32> {
		return None;
	}

}

impl<'a> Drawable for RenderedText<'a> {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		let dfont = ctx.default_font.clone();
		let font = self.font.unwrap_or(&dfont);
		let scale = self.size.map(|s| s / font.height()).unwrap_or(1.0);
		let gh = font.height() * scale + self.line_spacing;

		let offset = (self.align.as_pt() + vec2!(1)) * 0.5;
		let offset_pos = -offset * vec2!(self.width, self.height);

		ctx.push(mat4!()
			.t2(offset_pos)
		, |ctx| {

			for (y, line) in self.lines.iter().enumerate() {

				let mut x = 0.0;
				let ox = (self.width - line.width) * offset.x;

				for ch in line.text.chars() {

					if let Some((tex, quad)) = font.get(ch) {

						let gw = tex.width() as f32 * quad.w * scale;
						let italic = -self.italic.unwrap_or(0.0);
						let bold = self.bold.unwrap_or(0.0) + 1.0;

						ctx.draw_t(mat4!()
							.t2(vec2!(x + ox, y as f32 * gh))
							.s2(vec2!(scale))
							.skx(italic)
							.tx(gw * -italic * (1.0 - offset.x) * 0.5)
							.sx(bold)
						, &sprite(&tex)
							.offset(vec2!(-1))
							.quad(quad)
							.color(self.color)
						)?;

						x += gw;

					}

				}

			}

			return Ok(());

		})?;

		return Ok(());

	}

}

impl<'a> Drawable for Text<'a> {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		ctx.draw(&self.render(ctx))?;

		return Ok(());

	}

}

