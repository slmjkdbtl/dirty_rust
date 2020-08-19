// wengwengweng

varying vec3 v_pos;
varying vec3 v_normal;
varying vec2 v_uv;
varying vec4 v_color;

uniform sampler2D u_tex;
uniform vec4 u_color;

vec4 default_color() {
	return v_color * u_color * texture2D(u_tex, v_uv);
}

{{user}}

void main() {

	gl_FragColor = frag();

	if (gl_FragColor.a == 0.0) {
		discard;
	}

}

