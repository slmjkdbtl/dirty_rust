// wengwengweng

vec4 frag() {
	return v_color * texture2D(u_tex, v_uv);
}

