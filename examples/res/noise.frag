// wengwengweng

vec4 frag(
	sampler2D tex,
	vec2 tc,
	vec2 sc,
	vec2 size,
	float time) {

	float noise = fract(sin(dot(tc, vec2(12.9898, 78.233))) * 43.5453);

	return vec4(0, 0, 1, 1);
// 	return texture2D(tex, coord + vec2(noise * 0.01, noise * 0.01));

}

