// wengwengweng

uniform float size;
uniform vec2 resolution;
uniform vec4 filter;
uniform vec4 color;

vec4 frag(sampler2D tex, vec2 uv) {

	float px = size / resolution.x;
	float py = size / resolution.y;
	vec4 oc = texture2D(tex, uv);

	if (oc.a == 0.0) {

		vec2 pts[4];

		pts[0] = vec2(-px, 0.0);
		pts[1] = vec2(px, 0.0);
		pts[2] = vec2(0.0, -py);
		pts[3] = vec2(0.0, py);

		for (int i = 0; i < 4; i++) {
			vec2 pt = clamp(uv + pts[i], vec2(0.0), vec2(1.0));
			if (texture2D(tex, pt).a == 1.0) {
				return color;
			}
		}

	}

	return oc * filter;

}


