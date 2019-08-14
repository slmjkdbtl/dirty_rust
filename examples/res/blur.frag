// wengwengweng

uniform float radius;
uniform vec2 resolution;

vec4 frag(sampler2D tex, vec2 uv) {

	if (radius == 0.0) {
		return texture2D(tex, uv);
	}

	vec4 c = vec4(0.0);
	float rx = radius / resolution.x;
	float ry = radius / resolution.y;
	float count = 0.0;

	// TODO: slow
	for (float i = uv.x - rx; i < uv.x + rx; i += 1.0 / resolution.x) {
		for (float j = uv.y - ry; j < uv.y + ry; j += 1.0 / resolution.y) {
			count += 1.0;
			c += texture2D(tex, vec2(i, j));
		}
	}

	return c / count;

}

