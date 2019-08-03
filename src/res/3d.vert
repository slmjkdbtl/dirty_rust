// wengwengweng

attribute vec3 pos;

uniform mat4 proj;
uniform mat4 model;

void main() {
	gl_Position = proj * model * vec4(pos, 1.0);
}

