// wengwengweng

uniform float u_threshold;

vec4 frag() {

	vec4 oc = texture2D(u_tex, v_uv) * v_color;

	if (oc.r + oc.g + oc.b < u_threshold * 3.0) {
		discard;
	}

	return oc;

}

