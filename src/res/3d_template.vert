// wengwengweng

attribute vec3 vert;
attribute vec2 uv;
attribute vec4 color;

varying vec2 coord;
varying vec4 tint;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

###REPLACE###

void main() {
	coord = uv;
	tint = color;
	gl_Position = pos(projection, view, model, vec4(vert, 1.0));
}

