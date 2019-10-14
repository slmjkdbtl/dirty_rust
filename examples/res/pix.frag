// wengwengweng

uniform float size;
uniform vec2 resolution;

// TODO: this is causing some extra pixels to appear at screen edge
vec4 frag() {

	if (size <= 0.0) {
		return texture2D(u_tex, v_uv);
	}

	vec2 nsize = vec2(size / resolution.x, size / resolution.y);
	float x = floor(v_uv.x / nsize.x + 0.5);
	float y = floor(v_uv.y / nsize.y + 0.5);
	vec4 c = texture2D(u_tex, vec2(x, y) * nsize);

	return c * v_color;

}

