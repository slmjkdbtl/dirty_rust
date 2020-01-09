// wengwengweng

vec4 frag() {
	float l = (v_normal.x + v_normal.y + v_normal.z) / 3.0;
	return mix(v_color * u_color * texture2D(u_tex, v_uv), vec4(vec3(l), 1.0), 0.24);
}

