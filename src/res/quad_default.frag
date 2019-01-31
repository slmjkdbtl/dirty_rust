// wengwengweng

vec4 effect(sampler2D tex, vec2 coord, vec4 color) {
	return texture2D(tex, coord) * color;
}

