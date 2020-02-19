// wengwengweng

vec4 frag() {
	return texture(u_tex, v_uv) * v_color;
}

