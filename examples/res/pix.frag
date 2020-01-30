// wengwengweng

uniform float u_size;
uniform vec2 u_resolution;

// TODO: this is causing some extra pixels to appear at screen edge
vec4 frag() {

	if (u_size <= 0.0) {
		return texture2D(u_tex, v_uv);
	}

	vec2 n_size = vec2(u_size / u_resolution.x, u_size / u_resolution.y);
	float x = floor(v_uv.x / n_size.x + 0.5);
	float y = floor(v_uv.y / n_size.y + 0.5);
	vec4 c = texture2D(u_tex, vec2(x, y) * n_size);

	return c * v_color;

}

