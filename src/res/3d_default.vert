// wengwengweng

vec4 pos(mat4 proj, mat4 view, mat4 model, vec4 vert) {
	return proj * view * model * vert;
}

