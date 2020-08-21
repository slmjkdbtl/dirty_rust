// wengwengweng

uniform float u_time;
uniform sampler2D u_dtex;

vec4 frag() {
	vec4 map = texture2D(u_dtex, vec2(v_uv.x + u_time * 0.1, v_uv.y + u_time * 0.1));
	float displacement = ((map.r + map.g + map.b) / 3.0 - 0.5) * 0.1;
	return texture2D(u_tex, v_uv + vec2(displacement)) * v_color * u_color;
}

