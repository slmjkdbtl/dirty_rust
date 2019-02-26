// wengwengweng

vec4 frag(
	sampler2D tex,
	vec2 t_coord,
	vec2 s_coord,
	vec2 size,
	vec4 color,
	float time) {

	if (length(t_coord - vec2(0.5)) <= 0.5) {
		return vec4(t_coord.x, t_coord.y, 1, 1);
	}

	return vec4(0);

}

