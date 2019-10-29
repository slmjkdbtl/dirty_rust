// wengwengweng

#version 330

in vec3 v_uv;

uniform vec4 u_color;
uniform samplerCube u_tex;

out vec4 o_color;

void main() {
	o_color = texture(u_tex, v_uv) * u_color;
}

