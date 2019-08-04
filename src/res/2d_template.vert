// wengwengweng

attribute vec2 pos;
attribute vec2 uv;
attribute vec4 color;

varying vec2 v_uv;
varying vec4 v_color;

uniform mat4 proj;

###REPLACE###

void main() {

	v_color = color;
	v_uv = uv;
	gl_Position = vert(proj, vec4(pos, 0.0, 1.0));

}

