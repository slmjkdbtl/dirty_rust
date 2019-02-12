// wengwengweng

vec4 vert(mat4 proj, mat4 view, mat4 model, vec4 pos, float time) {
	return proj * view * model * pos;
}

