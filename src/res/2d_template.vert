// wengwengweng

attribute vec3 a_pos;
attribute vec2 a_uv;
attribute vec4 a_color;

varying vec2 v_uv;
varying vec4 v_color;

uniform mat4 u_proj;

###REPLACE###

void main() {

	v_color = a_color;
	v_uv = a_uv;
	gl_Position = vert();

}

