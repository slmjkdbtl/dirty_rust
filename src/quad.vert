// wengwengweng

attribute vec2 pos;
attribute vec2 uv;

varying vec2 tex_coord;

uniform mat4 trans;
uniform mat4 proj;
uniform vec4 quad;

void main() {
	tex_coord = quad.xy + uv * quad.zw;
	gl_Position = proj * trans * vec4(pos, 0.0, 1.0);
}

