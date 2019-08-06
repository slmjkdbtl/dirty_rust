// wengwengweng

// varying vec2 v_uv;
varying vec4 v_color;

uniform sampler2D tex;
uniform vec4 color;

###REPLACE###

void main() {
	gl_FragColor = v_color;
}

