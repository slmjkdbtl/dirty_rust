// wengwengweng

//! Global Resources

pub mod shader {

	#[cfg(web)]
	pub const TEMPLATE_VERT: &str = include_str!("shaders/es/3d_template.vert");
	#[cfg(web)]
	pub const DEFAULT_VERT: &str = include_str!("shaders/es/3d_default.vert");
	#[cfg(web)]
	pub const TEMPLATE_FRAG: &str = include_str!("shaders/es/3d_template.frag");
	#[cfg(web)]
	pub const DEFAULT_FRAG: &str = include_str!("shaders/es/3d_default.frag");
	#[cfg(web)]
	pub const CUBEMAP_VERT: &str = include_str!("shaders/es/cubemap.vert");
	#[cfg(web)]
	pub const CUBEMAP_FRAG: &str = include_str!("shaders/es/cubemap.frag");

	#[cfg(all(not(web), feature="gl3"))]
	pub const TEMPLATE_VERT: &str = include_str!("shaders/330/3d_template.vert");
	#[cfg(all(not(web), feature="gl3"))]
	pub const DEFAULT_VERT: &str = include_str!("shaders/330/3d_default.vert");
	#[cfg(all(not(web), feature="gl3"))]
	pub const TEMPLATE_FRAG: &str = include_str!("shaders/330/3d_template.frag");
	#[cfg(all(not(web), feature="gl3"))]
	pub const DEFAULT_FRAG: &str = include_str!("shaders/330/3d_default.frag");
	#[cfg(all(not(web), feature="gl3"))]
	pub const CUBEMAP_VERT: &str = include_str!("shaders/330/cubemap.vert");
	#[cfg(all(not(web), feature="gl3"))]
	pub const CUBEMAP_FRAG: &str = include_str!("shaders/330/cubemap.frag");

	#[cfg(all(not(web), not(feature="gl3")))]
	pub const TEMPLATE_VERT: &str = include_str!("shaders/120/3d_template.vert");
	#[cfg(all(not(web), not(feature="gl3")))]
	pub const DEFAULT_VERT: &str = include_str!("shaders/120/3d_default.vert");
	#[cfg(all(not(web), not(feature="gl3")))]
	pub const TEMPLATE_FRAG: &str = include_str!("shaders/120/3d_template.frag");
	#[cfg(all(not(web), not(feature="gl3")))]
	pub const DEFAULT_FRAG: &str = include_str!("shaders/120/3d_default.frag");
	#[cfg(all(not(web), not(feature="gl3")))]
	pub const CUBEMAP_VERT: &str = include_str!("shaders/120/cubemap.vert");
	#[cfg(all(not(web), not(feature="gl3")))]
	pub const CUBEMAP_FRAG: &str = include_str!("shaders/120/cubemap.frag");

}

pub mod font {

	use crate::gfx::BitmapFontData;

	// http://www.dsg4.com/04/extra/bitmap/index.html
	pub const F04B03: BitmapFontData = BitmapFontData {
		img: include_bytes!("fonts/04b03.png"),
		gw: 6,
		gh: 8,
		chars: r##" !"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~"##,
	};

	// https://en.wikipedia.org/wiki/Code_page_437
	pub const CP437: BitmapFontData = BitmapFontData {
		img: include_bytes!("fonts/CP437.png"),
		gw: 9,
		gh: 16,
		chars: r##" ☺☻♥♦♣♠•◘○◙♂♀♪♫☼►◄↕‼¶§▬↨↑↓→←∟↔▲▼ !"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~⌂ÇüéâäàåçêëèïîìÄÅÉæÆôöòûùÿÖÜ¢£¥₧ƒáíóúñÑªº¿⌐¬½¼¡«»░▒▓│┤╡╢╖╕╣║╗╝╜╛┐└┴┬├─┼╞╟╚╔╩╦╠═╬╧╨╤╥╙╘╒╓╫╪┘┌█▄▌▐▀αßΓπΣσµτΦΘΩδ∞φε∩≡±≥≤⌠⌡÷≈°∙·√ⁿ²■"##,
	};

	// http://upperbounds.net
	pub const PROGGY: BitmapFontData = BitmapFontData {
		img: include_bytes!("fonts/proggy.png"),
		gw: 7,
		gh: 13,
		chars: r##" !"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~"##,
	};

	// http://pelulamu.net/unscii/
	pub const UNSCII: BitmapFontData = BitmapFontData {
		img: include_bytes!("fonts/unscii.png"),
		gw: 8,
		gh: 8,
		chars: r##" !"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~"##,
	};

}

