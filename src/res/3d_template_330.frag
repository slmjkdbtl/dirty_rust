// wengwengweng

#version 330

// in vec2 v_uv;
in vec4 v_color;

out vec4 frag_color;

uniform sampler2D tex;
uniform vec4 color;

###REPLACE###

void main() {
	frag_color = v_color;
}

