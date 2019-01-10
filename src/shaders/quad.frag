// wengwengweng

varying vec2 tex_coord;
varying vec4 tint;

uniform sampler2D tex;

void main() {

	gl_FragColor = texture2D(tex, tex_coord) * tint;

}

