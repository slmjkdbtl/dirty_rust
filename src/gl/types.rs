// wengwengweng

macro_rules! bind_enum {

	($name:ident($type:ty) { $($member:ident => $dest:expr),+$(,)? }) => {

		#[allow(missing_docs)]
		#[derive(Clone, Copy, Debug, PartialEq)]
		pub enum $name {
			$($member,)+
		}

		impl From<$name> for $type {
			fn from(t: $name) -> $type {
				return match t {
					$($name::$member => $dest,)+
				}
			}
		}

	};

}

bind_enum!(BufferUsage(u32) {
	Static => glow::STATIC_DRAW,
	Dynamic => glow::DYNAMIC_DRAW,
	Stream => glow::STREAM_DRAW,
});

bind_enum!(FilterMode(i32) {
	Nearest => glow::NEAREST as i32,
	Linear => glow::LINEAR as i32,
});

bind_enum!(WrapMode(i32) {
	ClampToEdge => glow::CLAMP_TO_EDGE as i32,
	ClampToBorder => glow::CLAMP_TO_BORDER as i32,
	Repeat => glow::REPEAT as i32,
	MirroredRepeat => glow::MIRRORED_REPEAT as i32,
});

bind_enum!(Capability(u32) {
	Blend => glow::BLEND,
	CullFace => glow::CULL_FACE,
	DepthTest => glow::DEPTH_TEST,
	StencilTest => glow::STENCIL_TEST,
	ScissorTest => glow::SCISSOR_TEST,
});

bind_enum!(BlendFac(u32) {
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

bind_enum!(BlendOp(u32) {
	Add => glow::FUNC_ADD,
	Sub => glow::FUNC_SUBTRACT,
	ReverseSub => glow::FUNC_REVERSE_SUBTRACT,
	Max => glow::MAX,
	Min => glow::MIN,
});

bind_enum!(StencilOp(u32) {
	Keep => glow::KEEP,
	Zero => glow::ZERO,
	Replace => glow::REPLACE,
	Increment => glow::INCR,
	Decrement => glow::DECR,
	IncWrap => glow::INCR_WRAP,
	DecWrap => glow::DECR_WRAP,
	Invert => glow::INVERT,
});

bind_enum!(Face(u32) {
	Front => glow::FRONT,
	Back => glow::BACK,
	FrontAndBack => glow::FRONT_AND_BACK,
});

bind_enum!(CullMode(u32) {
	Clockwise => glow::CW,
	CounterClockwise => glow::CCW,
});

bind_enum!(Surface(u32) {
	Color => glow::COLOR_BUFFER_BIT,
	Stencil => glow::STENCIL_BUFFER_BIT,
	Depth => glow::DEPTH_BUFFER_BIT,
});

bind_enum!(Cmp(u32) {
	Never => glow::NEVER,
	Less => glow::LESS,
	LessOrEqual => glow::LEQUAL,
	Greater => glow::GREATER,
	GreaterOrEqual => glow::GEQUAL,
	Equal => glow::EQUAL,
	NotEqual => glow::NOTEQUAL,
	Always => glow::ALWAYS,
});

#[derive(Clone, Copy, PartialEq)]
pub enum Primitive {
	Point(f32),
	Line(f32),
	Triangle,
	LineStrip,
	TriangleFan,
	TriangleStrip,
}

impl From<Primitive> for u32 {

	fn from(p: Primitive) -> u32 {
		return match p {
			Primitive::Point(_) => glow::POINTS,
			Primitive::Line(_) => glow::LINES,
			Primitive::Triangle => glow::TRIANGLES,
			Primitive::LineStrip => glow::LINE_STRIP,
			Primitive::TriangleFan => glow::TRIANGLE_FAN,
			Primitive::TriangleStrip => glow::TRIANGLE_STRIP,
		};
	}

}

bind_enum!(TextureType(u32) {
	Tex2D => glow::TEXTURE_2D,
	Cubemap => glow::TEXTURE_CUBE_MAP,
});

bind_enum!(CubemapSide(u32) {
	Right => glow::TEXTURE_CUBE_MAP_POSITIVE_X,
	Left => glow::TEXTURE_CUBE_MAP_NEGATIVE_X,
	Up => glow::TEXTURE_CUBE_MAP_POSITIVE_Y,
	Down => glow::TEXTURE_CUBE_MAP_NEGATIVE_Y,
	Back => glow::TEXTURE_CUBE_MAP_POSITIVE_Z,
	Front => glow::TEXTURE_CUBE_MAP_NEGATIVE_Z,
});

bind_enum!(ShaderType(u32) {
	Vertex => glow::VERTEX_SHADER,
	Fragment => glow::FRAGMENT_SHADER,
});

