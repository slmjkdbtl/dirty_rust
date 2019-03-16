// wengwengweng

attribute vec2 pos;
attribute vec2 uv;
attribute vec4 color;

varying vec2 coord;
varying vec4 tint;

uniform mat4 projection;
uniform float time;

###REPLACE###

void main() {

	tint = color;
	coord = uv;
	gl_Position = vert(projection, vec4(pos, 1.0, 1.0), time);

}
