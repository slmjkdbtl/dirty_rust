// wengwengweng

vec4 effect(sampler2D tex, vec2 coord, vec2 s_coord, vec4 tint, float time) {

	float noise = fract(sin(dot(s_coord, vec2(12.9898, 78.233))) * 43.5453);

	return texture2D(tex, coord + vec2(noise * 0.01, noise * 0.01)) * tint;

}

