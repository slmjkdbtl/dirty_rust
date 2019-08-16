// wengwengweng

// varying vec2 v_uv;
varying float v_brightness;
varying vec4 v_color;

uniform sampler2D tex;
uniform vec4 color;

###REPLACE###

void main() {

// 	gl_FragColor = vec4(v_color.xyz * v_brightness, v_color.a);
	gl_FragColor = v_color;

	if (gl_FragColor.a == 0.0) {
		discard;
	}

}

