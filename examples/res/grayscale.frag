// wengwengweng

vec4 frag() {

	vec4 c = texture2D(u_tex, v_uv) * v_color;
	float brightness = (c.r + c.g + c.b) / 3.0;

	return vec4(vec3(brightness), c.a);

}

