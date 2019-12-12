// wengwengweng

pub mod shader {

	#[cfg(web)]
	pub const TEMPLATE_2D_VERT: &str = include_str!("shaders/es/2d_template.vert");
	#[cfg(web)]
	pub const DEFAULT_2D_VERT: &str = include_str!("shaders/es/2d_default.vert");
	#[cfg(web)]
	pub const TEMPLATE_2D_FRAG: &str = include_str!("shaders/es/2d_template.frag");
	#[cfg(web)]
	pub const DEFAULT_2D_FRAG: &str = include_str!("shaders/es/2d_default.frag");
	#[cfg(web)]
	pub const TEMPLATE_3D_VERT: &str = include_str!("shaders/es/3d_template.vert");
	#[cfg(web)]
	pub const DEFAULT_3D_VERT: &str = include_str!("shaders/es/3d_default.vert");
	#[cfg(web)]
	pub const TEMPLATE_3D_FRAG: &str = include_str!("shaders/es/3d_template.frag");
	#[cfg(web)]
	pub const DEFAULT_3D_FRAG: &str = include_str!("shaders/es/3d_default.frag");
	#[cfg(web)]
	pub const CUBEMAP_VERT: &str = include_str!("shaders/es/cubemap.vert");
	#[cfg(web)]
	pub const CUBEMAP_FRAG: &str = include_str!("shaders/es/cubemap.frag");

	#[cfg(all(not(web), feature="gl3"))]
	pub const TEMPLATE_2D_VERT: &str = include_str!("shaders/330/2d_template.vert");
	#[cfg(all(not(web), feature="gl3"))]
	pub const DEFAULT_2D_VERT: &str = include_str!("shaders/330/2d_default.vert");
	#[cfg(all(not(web), feature="gl3"))]
	pub const TEMPLATE_2D_FRAG: &str = include_str!("shaders/330/2d_template.frag");
	#[cfg(all(not(web), feature="gl3"))]
	pub const DEFAULT_2D_FRAG: &str = include_str!("shaders/330/2d_default.frag");
	#[cfg(all(not(web), feature="gl3"))]
	pub const TEMPLATE_3D_VERT: &str = include_str!("shaders/330/3d_template.vert");
	#[cfg(all(not(web), feature="gl3"))]
	pub const DEFAULT_3D_VERT: &str = include_str!("shaders/330/3d_default.vert");
	#[cfg(all(not(web), feature="gl3"))]
	pub const TEMPLATE_3D_FRAG: &str = include_str!("shaders/330/3d_template.frag");
	#[cfg(all(not(web), feature="gl3"))]
	pub const DEFAULT_3D_FRAG: &str = include_str!("shaders/330/3d_default.frag");
	#[cfg(all(not(web), feature="gl3"))]
	pub const CUBEMAP_VERT: &str = include_str!("shaders/330/cubemap.vert");
	#[cfg(all(not(web), feature="gl3"))]
	pub const CUBEMAP_FRAG: &str = include_str!("shaders/330/cubemap.frag");

	#[cfg(all(not(web), not(feature="gl3")))]
	pub const TEMPLATE_2D_VERT: &str = include_str!("shaders/120/2d_template.vert");
	#[cfg(all(not(web), not(feature="gl3")))]
	pub const DEFAULT_2D_VERT: &str = include_str!("shaders/120/2d_default.vert");
	#[cfg(all(not(web), not(feature="gl3")))]
	pub const TEMPLATE_2D_FRAG: &str = include_str!("shaders/120/2d_template.frag");
	#[cfg(all(not(web), not(feature="gl3")))]
	pub const DEFAULT_2D_FRAG: &str = include_str!("shaders/120/2d_default.frag");
	#[cfg(all(not(web), not(feature="gl3")))]
	pub const TEMPLATE_3D_VERT: &str = include_str!("shaders/120/3d_template.vert");
	#[cfg(all(not(web), not(feature="gl3")))]
	pub const DEFAULT_3D_VERT: &str = include_str!("shaders/120/3d_default.vert");
	#[cfg(all(not(web), not(feature="gl3")))]
	pub const TEMPLATE_3D_FRAG: &str = include_str!("shaders/120/3d_template.frag");
	#[cfg(all(not(web), not(feature="gl3")))]
	pub const DEFAULT_3D_FRAG: &str = include_str!("shaders/120/3d_default.frag");
	#[cfg(all(not(web), not(feature="gl3")))]
	pub const CUBEMAP_VERT: &str = include_str!("shaders/120/cubemap.vert");
	#[cfg(all(not(web), not(feature="gl3")))]
	pub const CUBEMAP_FRAG: &str = include_str!("shaders/120/cubemap.frag");

}

pub mod font {

	use super::super::app::gfx::BitmapFontData;

	// http://www.dsg4.com/04/extra/bitmap/index.html
	pub const F04B03: BitmapFontData = BitmapFontData::new(
		include_bytes!("fonts/04b03.png"),
		6,
		8,
		r##" !"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~"##,
	);

	// https://en.wikipedia.org/wiki/Code_page_437
	pub const CP437: BitmapFontData = BitmapFontData::new(
		include_bytes!("fonts/CP437.png"),
		9,
		16,
		r##" ☺☻♥♦♣♠•◘○◙♂♀♪♫☼►◄↕‼¶§▬↨↑↓→←∟↔▲▼ !"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~⌂ÇüéâäàåçêëèïîìÄÅÉæÆôöòûùÿÖÜ¢£¥₧ƒáíóúñÑªº¿⌐¬½¼¡«»░▒▓│┤╡╢╖╕╣║╗╝╜╛┐└┴┬├─┼╞╟╚╔╩╦╠═╬╧╨╤╥╙╘╒╓╫╪┘┌█▄▌▐▀αßΓπΣσµτΦΘΩδ∞φε∩≡±≥≤⌠⌡÷≈°∙·√ⁿ²■"##,
	);

	// http://upperbounds.net
	pub const PROGGY: BitmapFontData = BitmapFontData::new(
		include_bytes!("fonts/proggy.png"),
		7,
		13,
		r##" !"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~"##,
	);

	// http://pelulamu.net/unscii/
	pub const UNSCII: BitmapFontData = BitmapFontData::new(
		include_bytes!("fonts/unscii.png"),
		8,
		8,
		r##" !"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~"##,
	);

}

