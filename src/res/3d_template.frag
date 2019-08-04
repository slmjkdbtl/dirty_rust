// wengwengweng

// varying vec2 coord;
varying vec4 tint;
varying vec3 v_normal;

// uniform sampler2D tex;
// uniform float time;

###REPLACE###

void main() {
	gl_FragColor = vec4(1, 1, 1, 1) * tint;
// 	gl_FragColor = effect(tex, coord, gl_FragCoord.xyz, tint, time);
}

