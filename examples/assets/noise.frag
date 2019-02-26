// wengwengweng

vec4 frag(
	sampler2D tex,
	vec2 t_coord,
	vec2 s_coord,
	vec2 size,
	vec4 color,
	float time) {

	float noise = fract(sin(dot(s_coord, vec2(12.9898, 78.233))) * 43.5453);

	return texture2D(tex, coord + vec2(noise * 0.01, noise * 0.01)) * tint;

}

