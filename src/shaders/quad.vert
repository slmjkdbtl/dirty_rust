// wengwengweng

attribute vec3 pos;
attribute vec2 uv;
attribute vec4 color;

varying vec2 tex_coord;

uniform mat4 transform;
uniform mat4 projection;
uniform vec4 quad;

void main() {

	tex_coord = quad.xy + uv * quad.zw;
	gl_Position = projection * transform * vec4(pos, 1.0);

}

