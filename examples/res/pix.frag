// wengwengweng

uniform float size;
uniform vec2 resolution;

// TODO: this is causing some extra pixels to appear at screen edge
vec4 frag(sampler2D tex, vec2 uv) {

	if (size <= 0.0) {
		return texture(tex, uv);
	}

	vec2 nsize = vec2(size / resolution.x, size / resolution.y);
	float x = floor(uv.x / nsize.x + 0.5);
	float y = floor(uv.y / nsize.y + 0.5);
	vec4 c = texture(tex, vec2(x, y) * nsize);

	return c;

}

