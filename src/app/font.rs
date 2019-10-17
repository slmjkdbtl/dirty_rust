// wengwengweng

use std::collections::HashMap;

use super::*;
use super::gfx::*;

pub type CharMap = HashMap<char, Quad>;

pub trait Font {
	fn texture(&self) -> &Texture;
	fn map(&self) -> &CharMap;
}

/// bitmap font
#[derive(Clone, PartialEq)]
pub struct BitmapFont {

	pub(super) tex: Texture,
	pub(super) map: HashMap<char, Quad>,
	pub(super) quad_size: Vec2,
	grid_width: i32,
	grid_height: i32,

}

impl BitmapFont {

	/// creat a bitmap font from a texture, and grid of characters
	pub fn from_tex(tex: Texture, cols: usize, rows: usize, chars: &str) -> Result<Self> {

		let mut map = HashMap::new();
		let quad_size = vec2!(1.0 / cols as f32, 1.0 / rows as f32);
		let tw = tex.width() as i32;
		let th = tex.height() as i32;

		if (tw % cols as i32 != 0 || th % rows as i32 != 0) {
			return Err(Error::Gfx("bitmap font texture size or column / row count not correct".into()));
		}

		for (i, ch) in chars.chars().enumerate() {

			map.insert(ch, quad!(
				(i % cols) as f32 * quad_size.x,
				(i / cols) as f32 * quad_size.y,
				quad_size.x,
				quad_size.y
			));

		}

		return Ok(Self {

			tex: tex,
			map: map,
			quad_size: quad_size,
			grid_width: tw as i32 / cols as i32,
			grid_height: th as i32 / rows as i32,

		});

	}

	/// get width of a char
	pub fn width(&self) -> i32 {
		return self.grid_width;
	}

	/// get height of a char
	pub fn height(&self) -> i32 {
		return self.grid_height;
	}

}

impl Font for BitmapFont {
	fn texture(&self) -> &Texture {
		return &self.tex;
	}
	fn map(&self) -> &CharMap {
		return &self.map;
	}
}

/// truetype font
pub struct TruetypeFont {
	font: fontdue::Font,
	size: u32,
	cur_pt: Pos,
	pub(super) map: HashMap<char, Quad>,
	pub(super) tex: Texture,
}

impl TruetypeFont {

	/// parse a truetype file from bytes
	pub fn from_bytes(ctx: &Ctx, b: &[u8], size: u32) -> Result<Self> {

		let font = fontdue::Font::from_bytes(b)?;
		let (max_w, max_h) = (size * 32, size * 32);
		let tex = Texture::new(ctx, max_w, max_h)?;

		if size > 72 {
			return Err(Error::Gfx(format!("font size cannot exceed 72")));
		}

		return Ok(Self {
			font: font,
			size: size,
			map: HashMap::new(),
			cur_pt: pos!(0, 0),
			tex: tex,
		});

	}

	/// manually cache characters
	pub fn prepare(&mut self, s: &str) -> Result<()> {

		let (tw, th) = (self.tex.width(), self.tex.height());

		for ch in s.chars() {

			if self.map.get(&ch).is_none() {

				let (metrics, bitmap) = self.font.rasterize(ch, self.size as f32);
				let (w, h) = (metrics.width as i32, metrics.height as i32);

				let mut nbitmap = Vec::with_capacity(bitmap.len() * 4);

				for b in bitmap {
					nbitmap.extend_from_slice(&[b, b, b, b]);
				}

				let (mut x, mut y) = self.cur_pt.into();

				if x + w >= tw {
					x = 0;
					y += h;
				}

				if y >= th {
					return Err(Error::Gfx(format!("reached font texture size limit")))
				}

				self.tex.sub_data(x as u32, y as u32, w as u32, h as u32, &nbitmap);

				self.map.insert(ch, quad!(
					x as f32 / tw as f32,
					y as f32 / th as f32,
					w as f32 / tw as f32,
					h as f32 / th as f32,
				));

				x += w;
				self.cur_pt = pos!(x, y);

			}

		}

		return Ok(());

	}

	/// get width of a string
	pub fn width(&self, s: &str) -> i32 {
		return s
			.chars()
			.map(|c| self.map.get(&c))
			.flatten()
			.map(|q| (q.w * self.tex.width() as f32) as i32)
			.sum();
	}

	/// get height of a char
	pub fn height(&self) -> i32 {
		return self.size as i32;
	}

}

impl Font for TruetypeFont {
	fn texture(&self) -> &Texture {
		return &self.tex;
	}
	fn map(&self) -> &CharMap {
		return &self.map;
	}
}

