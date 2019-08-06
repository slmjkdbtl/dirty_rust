// wengwengweng

#[cfg(feature="gl3")]
const TEMPLATE_2D_VERT: &str = include_str!("2d_template_330.vert");
#[cfg(feature="gl3")]
const TEMPLATE_2D_FRAG: &str = include_str!("2d_template_330.frag");

#[cfg(feature="gl3")]
const DEFAULT_2D_VERT: &str = include_str!("2d_default_330.vert");
#[cfg(feature="gl3")]
const DEFAULT_2D_FRAG: &str = include_str!("2d_default_330.frag");

#[cfg(not(feature="gl3"))]
const TEMPLATE_2D_VERT: &str = include_str!("2d_template.vert");
#[cfg(not(feature="gl3"))]
const TEMPLATE_2D_FRAG: &str = include_str!("2d_template.frag");

#[cfg(not(feature="gl3"))]
const DEFAULT_2D_VERT: &str = include_str!("2d_default.vert");
#[cfg(not(feature="gl3"))]
const DEFAULT_2D_FRAG: &str = include_str!("2d_default.frag");

#[cfg(feature="gl3")]
const TEMPLATE_3D_VERT: &str = include_str!("3d_template_330.vert");
#[cfg(feature="gl3")]
const TEMPLATE_3D_FRAG: &str = include_str!("3d_template_330.frag");

#[cfg(feature="gl3")]
const DEFAULT_3D_VERT: &str = include_str!("3d_default_330.vert");
#[cfg(feature="gl3")]
const DEFAULT_3D_FRAG: &str = include_str!("3d_default_330.frag");

#[cfg(not(feature="gl3"))]
const TEMPLATE_3D_VERT: &str = include_str!("3d_template.vert");
#[cfg(not(feature="gl3"))]
const TEMPLATE_3D_FRAG: &str = include_str!("3d_template.frag");

#[cfg(not(feature="gl3"))]
const DEFAULT_3D_VERT: &str = include_str!("3d_default.vert");
#[cfg(not(feature="gl3"))]
const DEFAULT_3D_FRAG: &str = include_str!("3d_default.frag");

const DEFAULT_FONT_IMG: &[u8] = include_bytes!("CP437.png");
const DEFAULT_FONT_COLS: usize = 32;
const DEFAULT_FONT_ROWS: usize = 8;
const DEFAULT_FONT_CHARS: &str = r##" ☺☻♥♦♣♠•◘○◙♂♀♪♫☼►◄↕‼¶§▬↨↑↓→←∟↔▲▼ !"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~⌂ÇüéâäàåçêëèïîìÄÅÉæÆôöòûùÿÖÜ¢£¥₧ƒáíóúñÑªº¿⌐¬½¼¡«»░▒▓│┤╡╢╖╕╣║╗╝╜╛┐└┴┬├─┼╞╟╚╔╩╦╠═╬╧╨╤╥╙╘╒╓╫╪┘┌█▄▌▐▀αßΓπΣσµτΦΘΩδ∞φε∩≡±≥≤⌠⌡÷≈°∙·√ⁿ²■"##;

// const DEFAULT_FONT_IMG: &[u8] = include_bytes!("proggy.png");
// const DEFAULT_FONT_COLS: usize = 95;
// const DEFAULT_FONT_ROWS: usize = 1;
// const DEFAULT_FONT_CHARS: &str = r##" !"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~"##;

