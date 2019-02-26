// wengwengweng

vec4 circle(vec2 center, float radius, vec2 pos, vec3 color, float blur) {

	float dis = length(pos - center);
	float o = smoothstep(radius, radius - blur, dis);

	return vec4(color, o);

}

vec4 frag(
	sampler2D tex,
	vec2 t_coord,
	vec2 s_coord,
	vec2 size,
	vec4 color,
	float time) {

	float st = sin(time);
	float ct = cos(time);

	vec4 c = circle(
		vec2(0.5),
		0.5,
		t_coord,
		vec3(t_coord.x - st * 0.1, t_coord.y + ct * 0.1, 1),
		0.2
	);

	return c;

}

