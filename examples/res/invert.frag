// wengwengweng

vec4 frag(sampler2D tex, vec2 uv) {

	vec4 c = default_color();

	return vec4(vec3(1) - c.xyz, c.a);

}

