// wengwengweng

#version 330

in vec2 v_uv;
in vec4 v_color;

uniform sampler2D u_tex;

out vec4 o_color;

###REPLACE###

void main() {

	o_color = frag(u_tex, v_uv) * v_color;

	if (o_color.a == 0.0) {
		discard;
	}

}

