// wengwengweng

vec4 frag(
	sampler2D tex,
	vec2 t_coord,
	vec2 s_coord,
	vec2 size,
	vec4 color,
	float time) {

	return vec4(t_coord.x, t_coord.y, 0, 1);

}

