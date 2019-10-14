// wengwengweng

uniform float size;
uniform vec2 resolution;
uniform vec4 filter;
uniform vec4 color;

vec4 frag() {

	float px = size / resolution.x;
	float py = size / resolution.y;
	vec4 oc = texture2D(u_tex, v_uv);

	if (oc.a == 0.0) {

		vec2 pts[4];

		pts[0] = vec2(-px, 0.0);
		pts[1] = vec2(px, 0.0);
		pts[2] = vec2(0.0, -py);
		pts[3] = vec2(0.0, py);

		for (int i = 0; i < 4; i++) {
			vec2 pt = clamp(v_uv + pts[i], vec2(0.0), vec2(1.0));
			if (texture2D(u_tex, pt).a == 1.0) {
				return color;
			}
		}

	}

	return oc * filter * v_color;

}


