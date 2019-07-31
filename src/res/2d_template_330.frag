// wengwengweng

#version 330

in vec2 coord;
in vec4 tint;

uniform sampler2D tex;
uniform float time;
uniform vec2 size;
uniform float rnd;

out vec4 frag_color;

float rand(vec2 co) {
	return fract(sin(dot(co.xy, vec2(12.9898, 78.233))) * 43758.5453 * rnd);
}

###REPLACE###

void main() {
	frag_color = frag(tex, coord, gl_FragCoord.xy / size, size, time) * tint;
}

