// wengwengweng

float rand(vec2 co){
	return fract(sin(dot(co.xy, vec2(12.9898, 78.233))) * 43758.5453);
}

vec4 frag() {

	vec4 c = v_color * u_color * texture2D(u_tex, v_uv);
	float l = (v_normal.x + v_normal.y + v_normal.z) / 3.0;

	c = mix(c, vec4(vec3(l), 1.0), 0.24);

	if (rand(v_uv) > c.a) {
		discard;
	}

	c.a = 1.0;

	return c;

}

