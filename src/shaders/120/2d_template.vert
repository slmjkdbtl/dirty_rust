// wengwengweng

attribute vec3 a_pos;
attribute vec2 a_uv;
attribute vec4 a_color;

varying vec2 v_uv;
varying vec3 v_pos;
varying vec4 v_color;

uniform mat4 u_proj;
uniform mat4 u_view;

###REPLACE###

void main() {

	v_color = a_color;
	v_pos = a_pos;
	v_uv = a_uv;
	gl_Position = vert();

}

