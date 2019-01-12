// wengwengweng

attribute vec2 pos;
attribute vec2 uv;
attribute vec4 color;

varying vec2 tex_coord;
varying vec4 tint;

uniform mat4 transform;
uniform mat4 projection;
uniform vec4 quad;

void main() {

	tint = color;
	tex_coord = uv;
	gl_Position = projection * vec4(pos, 0.0, 1.0);

}
