// wengwengweng

#version 330 core

out vec4 o_color;
in vec2 v_uv;
in vec4 v_color;

uniform vec4 u_color;
uniform sampler2D u_texture;

void main() {
	o_color = texture(u_texture, v_uv) * v_color * u_color;
}

