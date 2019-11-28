// wengwengweng

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

use super::gfx::BitmapFontData;

pub const F04B03_DATA: BitmapFontData = BitmapFontData::new(
	include_bytes!("fonts/04b03.png"),
	6,
	8,
	r##" !"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~"##,
);

pub const CP437_DATA: BitmapFontData = BitmapFontData::new(
	include_bytes!("fonts/CP437.png"),
	9,
	16,
	r##" ☺☻♥♦♣♠•◘○◙♂♀♪♫☼►◄↕‼¶§▬↨↑↓→←∟↔▲▼ !"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~⌂ÇüéâäàåçêëèïîìÄÅÉæÆôöòûùÿÖÜ¢£¥₧ƒáíóúñÑªº¿⌐¬½¼¡«»░▒▓│┤╡╢╖╕╣║╗╝╜╛┐└┴┬├─┼╞╟╚╔╩╦╠═╬╧╨╤╥╙╘╒╓╫╪┘┌█▄▌▐▀αßΓπΣσµτΦΘΩδ∞φε∩≡±≥≤⌠⌡÷≈°∙·√ⁿ²■"##,
);

pub const PROGGY_DATA: BitmapFontData = BitmapFontData::new(
	include_bytes!("fonts/proggy.png"),
	7,
	13,
	r##" !"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~"##,
);

// pub const PROGGY_DATA: BitmapFontData = BitmapFontData::new(
// 	include_bytes!("fonts/proggy2.png"),
// 	8,
// 	14,
// 	r##" !"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~"##,
// );

