// wengwengweng

attribute vec3 vert;
attribute vec4 color;
varying vec4 tint;
uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

vec4 pos(mat4 proj, mat4 view, mat4 model, vec4 vert) {
	return proj * view * model * vert;
}

void main() {
	tint = color;
	gl_Position = pos(projection, view, model, vec4(vert, 1.0));
}

