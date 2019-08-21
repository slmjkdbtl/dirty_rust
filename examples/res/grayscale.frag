// wengwengweng

vec4 frag(sampler2D tex, vec2 uv) {

	vec4 c = texture2D(tex, uv);
	float brightness = (c.r + c.g + c.b) / 3.0;

	return vec4(vec3(brightness), c.a);

}

