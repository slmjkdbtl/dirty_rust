// wengwengweng

uniform sampler2D tex;
uniform vec4 tint;
varying vec2 tex_coord;

void main() {
	gl_FragColor = texture2D(tex, tex_coord) * tint;
}

