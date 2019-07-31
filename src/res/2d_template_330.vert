// wengwengweng

#version 330

layout(location = 0) in vec2 pos;
layout(location = 1) in vec2 uv;
layout(location = 2) in vec4 color;

out vec2 coord;
out vec4 tint;

uniform mat4 proj;
uniform float time;
uniform float rnd;

float rand(vec2 co) {
	return fract(sin(dot(co.xy, vec2(12.9898, 78.233))) * 43758.5453 * rnd);
}

###REPLACE###

void main() {

	tint = color;
	coord = uv;
	gl_Position = vert(proj, vec4(pos, 1.0, 1.0), time);

}

