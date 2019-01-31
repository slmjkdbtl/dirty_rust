// wengwengweng

attribute vec2 vert;
attribute vec2 uv;
attribute vec4 color;

varying vec2 coord;
varying vec4 tint;

uniform mat4 transform;
uniform mat4 projection;
uniform vec4 quad;

###REPLACE###

void main() {

	tint = color;
	coord = uv;
	gl_Position = pos(projection, vec4(vert, 1.0, 1.0));

}
