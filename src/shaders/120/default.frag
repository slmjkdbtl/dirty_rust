// wengwengweng

vec4 frag() {
	return v_color * u_color * texture2D(u_tex, v_uv);
}

