// wengwengweng

// varying vec2 v_uv;
varying vec4 v_color;
varying float v_brightness;

uniform sampler2D tex;

###REPLACE###

void main() {
	gl_FragColor = vec4(v_color.xzy * v_brightness, v_color.a);
}

