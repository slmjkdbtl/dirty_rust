// wengwengweng

use crate::gfx::BitmapFontData;

// http://www.dsg4.com/04/extra/bitmap/index.html
pub const F04B03: BitmapFontData = BitmapFontData {
	img: include_bytes!("04b03.png"),
	gw: 6,
	gh: 8,
	chars: r##" !"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~"##,
};

// https://en.wikipedia.org/wiki/Code_page_437
pub const CP437: BitmapFontData = BitmapFontData {
	img: include_bytes!("CP437.png"),
	gw: 9,
	gh: 16,
	chars: r##" ☺☻♥♦♣♠•◘○◙♂♀♪♫☼►◄↕‼¶§▬↨↑↓→←∟↔▲▼ !"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~⌂ÇüéâäàåçêëèïîìÄÅÉæÆôöòûùÿÖÜ¢£¥₧ƒáíóúñÑªº¿⌐¬½¼¡«»░▒▓│┤╡╢╖╕╣║╗╝╜╛┐└┴┬├─┼╞╟╚╔╩╦╠═╬╧╨╤╥╙╘╒╓╫╪┘┌█▄▌▐▀αßΓπΣσµτΦΘΩδ∞φε∩≡±≥≤⌠⌡÷≈°∙·√ⁿ²■"##,
};

// http://upperbounds.net
pub const PROGGY: BitmapFontData = BitmapFontData {
	img: include_bytes!("proggy.png"),
	gw: 7,
	gh: 13,
	chars: r##" !"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~"##,
};

// http://pelulamu.net/unscii/
pub const UNSCII: BitmapFontData = BitmapFontData {
	img: include_bytes!("unscii.png"),
	gw: 8,
	gh: 8,
	chars: r##" !"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~"##,
};

