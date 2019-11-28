#version 300 es
precision mediump float;

in vec2 v_uv;
in vec4 v_color;

uniform sampler2D u_tex;

out vec4 o_color;

###REPLACE###

void main() {

	o_color = frag();

	if (o_color.a == 0.0) {
		discard;
	}

}

// in vec2 vert;
// out vec4 color;
// void main() {
// 	color = vec4(vert, 0.5, 1.0);
// }

