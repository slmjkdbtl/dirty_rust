// wengwengweng

uniform float radius;
uniform vec2 dir;
uniform vec2 resolution;

vec4 frag(sampler2D tex, vec2 uv) {

	if (radius <= 0.0) {
		return texture2D(tex, uv);
	}

	vec4 c = vec4(0.0);
	vec2 dir = normalize(dir);

	for (float i = -radius; i <= radius; i += 1.0) {
		c += texture2D(tex, uv + dir * (i / resolution.x));
	}

	return c / (radius * 2.0);

}

