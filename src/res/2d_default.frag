// wengwengweng

vec4 frag() {
	return texture2D(u_tex, v_uv) * v_color;
}

