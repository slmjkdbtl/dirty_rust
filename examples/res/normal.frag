// wengwengweng

vec4 frag() {

	vec4 c = default_color();

	// normal
	float l = (v_normal.x + v_normal.y + v_normal.z) / 3.0;
	c = mix(c, vec4(vec3(l), 1.0), 0.24);

	return c;

}

