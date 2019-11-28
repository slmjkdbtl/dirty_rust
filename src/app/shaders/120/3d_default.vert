// wengwengweng

vec4 vert() {
	return u_proj * u_view * vec4(v_pos, 1.0);
}

