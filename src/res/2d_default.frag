// wengwengweng

vec4 frag(
	sampler2D tex,
	vec2 tc,
	vec2 sc,
	vec2 size,
	float time) {

	return texture2D(tex, tc);

}

