// wengwengweng

#version 330

in vec2 v_uv;
in vec4 v_color;

uniform sampler2D tex;

out vec4 frag_color;

###REPLACE###

void main() {
	frag_color = frag(tex, v_uv) * v_color;
}

