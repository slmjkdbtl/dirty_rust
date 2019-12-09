// wengwengweng

uniform float u_size;
uniform vec2 u_resolution;
uniform vec4 u_color;

vec4 frag() {

	vec4 oc = texture2D(u_tex, v_uv) * v_color;

	if (oc.a != 0.0 || u_size == 0.0 || u_color.a == 0.0) {
		return oc;
	}

	float px = u_size / u_resolution.x;
	float py = u_size / u_resolution.y;

	vec2 pts[4];

	pts[0] = vec2(-px, 0.0);
	pts[1] = vec2(px, 0.0);
	pts[2] = vec2(0.0, -py);
	pts[3] = vec2(0.0, py);

	for (int i = 0; i < 4; i++) {
		vec2 pt = clamp(v_uv + pts[i], vec2(0.0), vec2(1.0));
		if (texture2D(u_tex, pt).a == 1.0) {
			return u_color;
		}
	}

	return oc;

}

