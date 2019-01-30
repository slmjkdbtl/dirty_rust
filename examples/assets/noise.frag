// wengwengweng

varying vec2 tex_coord;
varying vec4 tint;

uniform sampler2D tex;

void main() {
	float noise = fract(sin(dot(gl_FragCoord.xy, vec2(12.9898,78.233))) * 43.5453);

	gl_FragColor = texture2D(tex, tex_coord + vec2(noise * 0.01, noise * 0.01)) * tint;
}

