// wengwengweng

vec4 frag() {

	vec4 c = v_color * u_color * texture2D(u_tex, v_uv);
	float l = (v_normal.x + v_normal.y + v_normal.z) / 3.0;

	return mix(c, vec4(vec3(l), 1.0), 0.24);

}

