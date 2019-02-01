// wengwengweng

attribute vec3 vert;
attribute vec3 color;
varying vec3 tint;
uniform mat4 model;
uniform mat4 projection;

void main() {
	tint = color;
	gl_Position = projection * model * vec4(vert, 1.0);
}

