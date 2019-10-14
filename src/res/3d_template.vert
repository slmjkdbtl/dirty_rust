// wengwengweng

attribute vec3 a_pos;
attribute vec3 a_normal;
attribute vec2 a_uv;
attribute vec4 a_color;

varying vec2 v_uv;
varying vec4 v_color;
varying vec3 v_normal;

uniform mat4 u_model;
uniform mat4 u_view;
uniform mat4 u_proj;

###REPLACE###

void main() {

	v_uv = a_uv;
	v_color = a_color;
	v_normal = a_normal;
	gl_Position = vert();

}

