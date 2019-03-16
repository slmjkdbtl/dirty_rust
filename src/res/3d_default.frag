// wengwengweng

vec4 effect(sampler2D tex, vec2 coord, vec3 s_coord, vec4 color, float time) {
	return texture2D(tex, coord) * color;
}

