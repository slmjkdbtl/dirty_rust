// wengwengweng

varying vec2 coord;
varying vec4 tint;

uniform sampler2D tex;
uniform float time;
uniform vec2 size;

###REPLACE###

void main() {
	gl_FragColor = frag(tex, coord, gl_FragCoord.xy / size, size, tint, time);
}

