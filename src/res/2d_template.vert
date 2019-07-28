// wengwengweng

attribute vec2 pos;
attribute vec2 uv;
attribute vec4 color;

varying vec2 coord;
varying vec4 tint;

uniform mat4 projection;
uniform float time;
uniform float rnd;

float rand(vec2 co) {
	return fract(sin(dot(co.xy, vec2(12.9898, 78.233))) * 43758.5453 * rnd);
}

###REPLACE###

void main() {

	tint = color;
	coord = uv;
	gl_Position = vert(projection, vec4(pos, 1.0, 1.0), time);

}

