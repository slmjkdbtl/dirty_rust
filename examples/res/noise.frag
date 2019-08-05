// wengwengweng

vec4 frag(sampler2D tex, vec2 uv) {

// 	if (size == 0) {
// 		return texture2D(tex, uv) * color;
// 	}

	float size = 0.01;

	float x = floor(uv.x / size + 0.5);
	float y = floor(uv.y / size + 0.5);
	vec4 c = texture2D(tex, vec2(x, y) * size);

// 	return texture2D(tex, uv);
	return c;

}

