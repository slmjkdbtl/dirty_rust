// wengwengweng

use super::*;

const ASCII_CHARS: &str = r##" !"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~"##;

#[derive(Clone, PartialEq)]
pub struct Char {
	pub ch: char,
	pub tex: Texture,
	pub quad: Quad,
	pub bearing_x: f32,
	pub bearing_y: f32,
	pub advance: f32,
}

/// Describes Features of a Font
pub trait Font {
	/// get render information of a character
	fn get(&self, ch: char) -> Option<&Char>;
	/// character height
	fn height(&self) -> f32;
	/// if there's a fixed character width
	fn width(&self) -> Option<f32>;
}

/// Data for Creating [`BitmapFont`](struct.BitmapFont.html)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BitmapFontData {
	pub img: &'static [u8],
	pub gw: u8,
	pub gh: u8,
	pub chars: &'static str,
}

impl BitmapFontData {
	pub const fn new(img: &'static [u8], gw: u8, gh: u8, chars: &'static str) -> Self {
		return Self {
			img,
			gw,
			gh,
			chars,
		};
	}
}

/// Bitmap Font
#[derive(Clone, PartialEq)]
pub struct BitmapFont {
	tex: Texture,
	map: HashMap<char, Char>,
	quad_size: Vec2,
	grid_width: u8,
	grid_height: u8,
}

impl BitmapFont {

	/// create bitmap font from a [`BitmapFontData`](struct.BitmapFontData)
	pub fn from_data(ctx: &impl GLCtx, data: BitmapFontData) -> Result<Self> {

		let font_tex = Texture::from_bytes(ctx, &data.img)?;

		return Ok(Self::from_tex(
			font_tex,
			data.gw,
			data.gh,
			data.chars,
		)?);

	}

	/// create bitmap font from a texture and parameters
	pub fn from_tex(tex: Texture, gw: u8, gh: u8, chars: &'static str) -> Result<Self> {

		let mut map = HashMap::new();
		let tw = tex.width();
		let th = tex.height();
		let quad_size = vec2!(gw as f32 / tw as f32, gh as f32 / th as f32);
		let cols = tw / gw as i32;

		if (tw % gw as i32 != 0 || th % gh as i32 != 0) {
			return Err(format!("bitmap font grid size not correct"));
		}

		for (i, ch) in chars.chars().enumerate() {

			map.insert(ch, Char {
				ch: ch,
				tex: tex.clone(),
				quad: quad!(
					(i as i32 % cols) as f32 * quad_size.x,
					(i as i32 / cols) as f32 * quad_size.y,
					quad_size.x,
					quad_size.y
				),
				bearing_x: 0.0,
				bearing_y: 0.0,
				advance: 0.0,
			});

		}

		return Ok(Self {
			tex: tex,
			map: map,
			quad_size,
			grid_width: gw,
			grid_height: gh,
		});

	}

	/// get width for a single char
	pub fn width(&self) -> i32 {
		return self.grid_width as i32;
	}

}

impl Font for BitmapFont {
	fn get(&self, ch: char) -> Option<&Char> {
		return self.map.get(&ch);
	}
	fn height(&self) -> f32 {
		return self.grid_height as f32;
	}
	fn width(&self) -> Option<f32> {
		return Some(self.grid_width as f32);
	}
}

// TODO
/// Font Loaded from a Truetype File
pub struct TruetypeFont {
	font: fontdue::Font,
	size: i32,
	cur_pt: Pt,
	map: HashMap<char, Char>,
	tex: Texture,
}

impl TruetypeFont {

	/// load from bytes of a truetype font file
	pub fn from_bytes(ctx: &impl GLCtx, b: &[u8], size: i32) -> Result<Self> {

		let font = fontdue::Font::from_bytes(b, fontdue::FontSettings::default())?;
		let (max_w, max_h) = (size * 32, size * 32);
		// TODO: make sure this doesn't exceed 2048x2048
		let tex = Texture::new(ctx, max_w, max_h)?;

		if size > 72 {
			return Err(format!("font size cannot exceed 72"));
		}

		return Ok(Self {
			font,
			size,
			map: HashMap::new(),
			cur_pt: pt!(0, 0),
			tex,
		});

	}

	/// cache characters to the texture
	pub fn cache(&mut self, ch: char) -> Result<()> {

		if self.map.get(&ch).is_some() {
			return Ok(());
		}

		let (tw, th) = (self.tex.width(), self.tex.height());
		let (metrics, bitmap) = self.font.rasterize(ch, self.size as f32);
		let (w, h) = (metrics.width as i32, metrics.height as i32);
		let mut nbitmap = Vec::with_capacity(bitmap.len() * 4);

		for b in bitmap {
			nbitmap.extend_from_slice(&[255, 255, 255, b]);
		}

		let (mut x, mut y) = (self.cur_pt.x, self.cur_pt.y);

		if x + w >= tw {
			x = 0;
			y += h;
		}

		if y >= th {
			return Err(format!("reached font texture size limit"));
		}

		self.tex.sub_data(x as i32, y as i32, w as i32, self.size as i32, &nbitmap);

		self.map.insert(ch, Char {
			ch: ch,
			quad: quad!(
				x as f32 / tw as f32,
				y as f32 / th as f32,
				w as f32 / tw as f32,
				h as f32 / th as f32,
			),
			tex: self.tex.clone(),
			bearing_x: 0.0,
			bearing_y: 0.0,
			advance: 0.0,
		});

		x += w;
		self.cur_pt = pt!(x, y);

		return Ok(());

	}

	/// cache a whole string
	pub fn cache_str(&mut self, s: &str) -> Result<()> {

		for ch in s.chars() {
			self.cache(ch)?;
		}

		return Ok(());

	}

	/// cache all ASCII chars
	pub fn cache_ascii(&mut self) -> Result<()> {
		return self.cache_str(ASCII_CHARS);
	}

	/// get width for a piece of string
	pub fn width(&self, s: &str) -> f32 {
		return s
			.chars()
			.map(|c| self.map.get(&c))
			.flatten()
			.map(|c| c.quad.w * self.tex.width() as f32)
			.sum();
	}

}

impl Font for TruetypeFont {
	fn get(&self, ch: char) -> Option<&Char> {
		return self.map.get(&ch);
	}
	fn height(&self) -> f32 {
		return self.size as f32;
	}
	fn width(&self) -> Option<f32> {
		return None;
	}
}

// TODO: 3d extruded text

