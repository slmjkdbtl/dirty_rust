// wengwengweng

varying vec2 v_uv;
varying vec4 v_color;

uniform sampler2D u_tex;

###REPLACE###

void main() {

	gl_FragColor = frag();

	if (gl_FragColor.a == 0.0) {
		discard;
	}

}

