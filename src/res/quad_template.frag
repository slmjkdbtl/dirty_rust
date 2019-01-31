// wengwengweng

varying vec2 coord;
varying vec4 tint;

uniform sampler2D tex;

###REPLACE###

void main() {
	gl_FragColor = effect(tex, coord, tint);
}

