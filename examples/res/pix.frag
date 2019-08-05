// wengwengweng

uniform float size;

vec4 frag(sampler2D tex, vec2 uv) {

	if (size == 0.0) {
		return texture2D(tex, uv);
	}

	float x = floor(uv.x / size + 0.5);
	float y = floor(uv.y / size + 0.5);
	vec4 c = texture2D(tex, vec2(x, y) * size);

	return c;

}

