// wengwengweng

varying vec2 tex_coord;

uniform sampler2D tex;
uniform vec4 tint;

void main() {

	gl_FragColor = texture2D(tex, tex_coord) * tint;

}
