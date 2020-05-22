// wengwengweng

//! Default Resources

pub(crate) mod shader {
	pub const TEMPLATE_VERT: &str = include_str!("shaders/template.vert");
	pub const DEFAULT_VERT: &str = include_str!("shaders/default.vert");
	pub const TEMPLATE_FRAG: &str = include_str!("shaders/template.frag");
	pub const DEFAULT_FRAG: &str = include_str!("shaders/default.frag");
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

