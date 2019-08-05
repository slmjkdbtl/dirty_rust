// wengwengweng

#version 330

// varying vec2 v_uv;

out vec4 frag_color;

uniform sampler2D tex;
uniform vec4 color;

###REPLACE###

void main() {
	frag_color = color;
}

