// wengwengweng

uniform float radius;
uniform vec2 resolution;

// TODO: this is causing some extra pixels to appear at screen edge
vec4 frag(sampler2D tex, vec2 uv) {

	if (radius == 0.0) {
		return texture2D(tex, uv);
	}

	return texture2D(tex, uv);

}

