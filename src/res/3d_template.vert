// wengwengweng

attribute vec3 pos;
attribute vec3 normal;
attribute vec4 color;

// varying vec2 coord;
varying vec4 tint;
varying vec3 v_normal;

uniform mat4 model;
uniform mat4 view;
uniform mat4 proj;
uniform float time;

###REPLACE###

void main() {
// 	coord = uv;
	tint = color;
	gl_Position = vert(proj, view, model, vec4(pos, 1.0));
}

