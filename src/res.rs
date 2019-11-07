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

pub const CP437_IMG: &[u8] = include_bytes!("fonts/CP437.png");
pub const CP437_COLS: usize = 32;
pub const CP437_ROWS: usize = 8;
pub const CP437_CHARS: &str = r##" ☺☻♥♦♣♠•◘○◙♂♀♪♫☼►◄↕‼¶§▬↨↑↓→←∟↔▲▼ !"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~⌂ÇüéâäàåçêëèïîìÄÅÉæÆôöòûùÿÖÜ¢£¥₧ƒáíóúñÑªº¿⌐¬½¼¡«»░▒▓│┤╡╢╖╕╣║╗╝╜╛┐└┴┬├─┼╞╟╚╔╩╦╠═╬╧╨╤╥╙╘╒╓╫╪┘┌█▄▌▐▀αßΓπΣσµτΦΘΩδ∞φε∩≡±≥≤⌠⌡÷≈°∙·√ⁿ²■"##;

pub const PROGGY_IMG: &[u8] = include_bytes!("fonts/proggy.png");
pub const PROGGY_COLS: usize = 95;
pub const PROGGY_ROWS: usize = 1;
pub const PROGGY_CHARS: &str = r##" !"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~"##;

// pub const PROGGY_IMG: &[u8] = include_bytes!("fonts/proggy2.png");
// pub const PROGGY_COLS: usize = 95;
// pub const PROGGY_ROWS: usize = 1;
// pub const PROGGY_CHARS: &str = r##" !"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~"##;

