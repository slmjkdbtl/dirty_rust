// wengwengweng

#[cfg(feature="gl3")]
pub const TEMPLATE_2D_VERT: &str = include_str!("2d_template_330.vert");
#[cfg(feature="gl3")]
pub const TEMPLATE_2D_FRAG: &str = include_str!("2d_template_330.frag");

#[cfg(feature="gl3")]
pub const DEFAULT_2D_VERT: &str = include_str!("2d_default_330.vert");
#[cfg(feature="gl3")]
pub const DEFAULT_2D_FRAG: &str = include_str!("2d_default_330.frag");

#[cfg(not(feature="gl3"))]
pub const TEMPLATE_2D_VERT: &str = include_str!("2d_template.vert");
#[cfg(not(feature="gl3"))]
pub const TEMPLATE_2D_FRAG: &str = include_str!("2d_template.frag");

#[cfg(not(feature="gl3"))]
pub const DEFAULT_2D_VERT: &str = include_str!("2d_default.vert");
#[cfg(not(feature="gl3"))]
pub const DEFAULT_2D_FRAG: &str = include_str!("2d_default.frag");

#[cfg(feature="gl3")]
pub const TEMPLATE_3D_VERT: &str = include_str!("3d_template_330.vert");
#[cfg(feature="gl3")]
pub const TEMPLATE_3D_FRAG: &str = include_str!("3d_template_330.frag");

#[cfg(feature="gl3")]
pub const DEFAULT_3D_VERT: &str = include_str!("3d_default_330.vert");
#[cfg(feature="gl3")]
pub const DEFAULT_3D_FRAG: &str = include_str!("3d_default_330.frag");

#[cfg(not(feature="gl3"))]
pub const TEMPLATE_3D_VERT: &str = include_str!("3d_template.vert");
#[cfg(not(feature="gl3"))]
pub const TEMPLATE_3D_FRAG: &str = include_str!("3d_template.frag");

#[cfg(not(feature="gl3"))]
pub const DEFAULT_3D_VERT: &str = include_str!("3d_default.vert");
#[cfg(not(feature="gl3"))]
pub const DEFAULT_3D_FRAG: &str = include_str!("3d_default.frag");

pub const CP437_IMG: &[u8] = include_bytes!("CP437.png");
pub const CP437_COLS: usize = 32;
pub const CP437_ROWS: usize = 8;
pub const CP437_CHARS: &str = r##" ☺☻♥♦♣♠•◘○◙♂♀♪♫☼►◄↕‼¶§▬↨↑↓→←∟↔▲▼ !"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~⌂ÇüéâäàåçêëèïîìÄÅÉæÆôöòûùÿÖÜ¢£¥₧ƒáíóúñÑªº¿⌐¬½¼¡«»░▒▓│┤╡╢╖╕╣║╗╝╜╛┐└┴┬├─┼╞╟╚╔╩╦╠═╬╧╨╤╥╙╘╒╓╫╪┘┌█▄▌▐▀αßΓπΣσµτΦΘΩδ∞φε∩≡±≥≤⌠⌡÷≈°∙·√ⁿ²■"##;

pub const PROGGY_IMG: &[u8] = include_bytes!("proggy.png");
pub const PROGGY_COLS: usize = 95;
pub const PROGGY_ROWS: usize = 1;
pub const PROGGY_CHARS: &str = r##" !"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~"##;

// pub const PROGGY_IMG: &[u8] = include_bytes!("proggy2.png");
// pub const PROGGY_COLS: usize = 95;
// pub const PROGGY_ROWS: usize = 1;
// pub const PROGGY_CHARS: &str = r##" !"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~"##;

