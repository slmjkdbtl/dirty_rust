// wengwengweng

uniform sampler2D u_tex2;

vec4 frag() {
	return texture2D(u_tex2, vec2(v_uv.x, 1.0 - v_uv.y));
}

