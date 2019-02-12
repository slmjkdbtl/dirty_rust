// wengwengweng

attribute vec3 pos;
attribute vec2 uv;
attribute vec4 color;

varying vec2 coord;
varying vec4 tint;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;
uniform float time;

###REPLACE###

void main() {
	coord = uv;
	tint = color;
	gl_Position = vert(projection, view, model, vec4(pos, 1.0), time);
}

