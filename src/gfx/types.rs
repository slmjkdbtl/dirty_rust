// wengwengweng

use crate::*;
use super::*;

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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Primitive {
	Point(f32),
	Line(f32),
	Triangle,
	LineStrip,
	TriangleFan,
	TriangleStrip,
}

impl Primitive {

	pub(super) fn as_glow(&self) -> u32 {
		return match self {
			Primitive::Point(_) => glow::POINTS,
			Primitive::Line(_) => glow::LINES,
			Primitive::Triangle => glow::TRIANGLES,
			Primitive::LineStrip => glow::LINE_STRIP,
			Primitive::TriangleFan => glow::TRIANGLE_FAN,
			Primitive::TriangleStrip => glow::TRIANGLE_STRIP,
		};
	}

}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Flip {
	None,
	X,
	Y,
	XY,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Blend {
	Alpha,
	Add,
	Replace,
}

impl Blend {
	pub(super) fn as_glow(&self) -> (BlendFac, BlendFac) {
		return match self {
			Blend::Alpha => (BlendFac::SrcAlpha, BlendFac::OneMinusSrcAlpha),
			Blend::Add => (BlendFac::SrcAlpha, BlendFac::DestAlpha),
			Blend::Replace => (BlendFac::SrcAlpha, BlendFac::Zero),
		};
	}
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Origin {
	TopLeft,
	Top,
	TopRight,
	Left,
	Center,
	Right,
	BottomLeft,
	Bottom,
	BottomRight,
}

impl Origin {

	pub fn as_pt(&self) -> Vec2 {

		use Origin::*;

		return match self {
			TopLeft => vec2!(-1, 1),
			Top => vec2!(0, 1),
			TopRight => vec2!(1, 1),
			Left => vec2!(-1, 0),
			Center => vec2!(0, 0),
			Right => vec2!(1, 0),
			BottomLeft => vec2!(-1, -1),
			Bottom => vec2!(0, -1),
			BottomRight => vec2!(1, -1),
		};

	}

}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct BlendDesc {
	pub src: BlendFac,
	pub dest: BlendFac,
	pub op: BlendOp,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct BlendState {
	pub color: BlendDesc,
	pub alpha: BlendDesc,
}

/// Describes a Stencil Operation
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StencilState {
	pub cmp: Cmp,
	pub sfail: StencilOp,
	pub dpfail: StencilOp,
	pub dppass: StencilOp,
}

// TODO: use this
#[derive(Clone, PartialEq)]
pub(super) struct RenderState<'a, U: UniformLayout> {
	pub prim: Primitive,
	pub stencil: Option<StencilState>,
	pub uniform: &'a U,
	pub blend: BlendState,
	pub canvas: Option<&'a Canvas>,
}

