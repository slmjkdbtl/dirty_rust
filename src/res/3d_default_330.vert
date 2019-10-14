// wengwengweng

vec4 vert() {
	vec3 world_pos = (u_model * vec4(a_pos, 1.0)).xyz;
	return u_proj * u_view * vec4(world_pos, 1.0);
}

