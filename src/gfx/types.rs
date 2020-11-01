// wengwengweng

use super::*;

macro_rules! make_handle {

	($t:ident, $lt:ident) => {

		paste::paste! {

			pub(super) struct [<$t Handle>] {
				gl: Rc<glow::Context>,
				id: glow::$t,
			}

			impl [<$t Handle>] {
				pub fn new(gl: &Rc<glow::Context>) -> Result<Self> {
					unsafe {
						return Ok(Self {
							id: gl.[<create_ $lt>]()?,
							gl: gl.clone(),
						});
					}
				}
				pub fn id(&self) -> glow::$t {
					return self.id;
				}
			}

			impl Drop for [<$t Handle>] {
				fn drop(&mut self) {
					unsafe {
						self.gl.[<delete_ $lt>](self.id);
					}
				}
			}

			impl PartialEq for [<$t Handle>] {
				fn eq(&self, other: &Self) -> bool {
					return self.id == other.id;
				}
			}

		}

	}

}

make_handle!(Buffer, buffer);
make_handle!(Texture, texture);
make_handle!(Program, program);
make_handle!(Framebuffer, framebuffer);
make_handle!(Renderbuffer, renderbuffer);

macro_rules! bind_enum {

	($vis:vis, $name:ident($type:ty) { $($member:ident => $dest:expr),+$(,)? }) => {

		#[allow(missing_docs)]
		#[derive(Clone, Copy, Debug, PartialEq)]
		$vis enum $name {
			$($member,)+
		}

		impl $name {
			pub(super) fn as_glow(&self) -> $type {
				return match self {
					$($name::$member => $dest,)+
				};
			}
		}

	};

}

bind_enum!(pub, FilterMode(i32) {
	Nearest => glow::NEAREST as i32,
	Linear => glow::LINEAR as i32,
});

bind_enum!(pub, WrapMode(i32) {
	ClampToEdge => glow::CLAMP_TO_EDGE as i32,
	ClampToBorder => glow::CLAMP_TO_BORDER as i32,
	Repeat => glow::REPEAT as i32,
	MirroredRepeat => glow::MIRRORED_REPEAT as i32,
});

bind_enum!(pub, Surface(u32) {
	Color => glow::COLOR_BUFFER_BIT,
	Stencil => glow::STENCIL_BUFFER_BIT,
	Depth => glow::DEPTH_BUFFER_BIT,
});

bind_enum!(pub, Cmp(u32) {
	Never => glow::NEVER,
	Less => glow::LESS,
	LessOrEqual => glow::LEQUAL,
	Greater => glow::GREATER,
	GreaterOrEqual => glow::GEQUAL,
	Equal => glow::EQUAL,
	NotEqual => glow::NOTEQUAL,
	Always => glow::ALWAYS,
});

bind_enum!(pub, StencilOp(u32) {
	Keep => glow::KEEP,
	Zero => glow::ZERO,
	Replace => glow::REPLACE,
	Increment => glow::INCR,
	Decrement => glow::DECR,
	IncWrap => glow::INCR_WRAP,
	DecWrap => glow::DECR_WRAP,
	Invert => glow::INVERT,
});

bind_enum!(pub(super), BufferUsage(u32) {
	Static => glow::STATIC_DRAW,
	Dynamic => glow::DYNAMIC_DRAW,
	Stream => glow::STREAM_DRAW,
});

bind_enum!(pub(super), Capability(u32) {
	Blend => glow::BLEND,
	CullFace => glow::CULL_FACE,
	DepthTest => glow::DEPTH_TEST,
	StencilTest => glow::STENCIL_TEST,
	ScissorTest => glow::SCISSOR_TEST,
	MultiSample => glow::MULTISAMPLE,
});

bind_enum!(pub(super), BlendFac(u32) {
	Zero => glow::ZERO,
	One => glow::ONE,
	SrcColor => glow::SRC_COLOR,
	OneMinusSrcColor => glow::ONE_MINUS_SRC_COLOR,
	DestColor => glow::DST_COLOR,
	OneMinusDestColor => glow::ONE_MINUS_DST_COLOR,
	SrcAlpha => glow::SRC_ALPHA,
	OneMinusSrcAlpha => glow::ONE_MINUS_SRC_ALPHA,
	DestAlpha => glow::DST_ALPHA,
	OneMinusDestAlpha => glow::ONE_MINUS_DST_ALPHA,
	SrcAlphaSaturate => glow::SRC_ALPHA_SATURATE,
	ConstantColor => glow::CONSTANT_COLOR,
	OneMinusConstantColor => glow::ONE_MINUS_CONSTANT_COLOR,
	ConstantAlpha => glow::CONSTANT_ALPHA,
	OneMinusConstantAlpha => glow::ONE_MINUS_CONSTANT_ALPHA,
});

bind_enum!(pub(super), BlendOp(u32) {
	Add => glow::FUNC_ADD,
	Sub => glow::FUNC_SUBTRACT,
	ReverseSub => glow::FUNC_REVERSE_SUBTRACT,
	Max => glow::MAX,
	Min => glow::MIN,
});

bind_enum!(pub(super), Face(u32) {
	Front => glow::FRONT,
	Back => glow::BACK,
	FrontAndBack => glow::FRONT_AND_BACK,
});

bind_enum!(pub(super), CullMode(u32) {
	Clockwise => glow::CW,
	CounterClockwise => glow::CCW,
});

bind_enum!(pub(super), TextureType(u32) {
	Tex2D => glow::TEXTURE_2D,
	Cubemap => glow::TEXTURE_CUBE_MAP,
});

bind_enum!(pub(super), CubemapSide(u32) {
	Right => glow::TEXTURE_CUBE_MAP_POSITIVE_X,
	Left => glow::TEXTURE_CUBE_MAP_NEGATIVE_X,
	Up => glow::TEXTURE_CUBE_MAP_POSITIVE_Y,
	Down => glow::TEXTURE_CUBE_MAP_NEGATIVE_Y,
	Back => glow::TEXTURE_CUBE_MAP_POSITIVE_Z,
	Front => glow::TEXTURE_CUBE_MAP_NEGATIVE_Z,
});

bind_enum!(pub(super), ShaderType(u32) {
	Vertex => glow::VERTEX_SHADER,
	Fragment => glow::FRAGMENT_SHADER,
});

