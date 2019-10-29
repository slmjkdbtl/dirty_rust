// wengwengweng

#version 330

layout(location = 0) in vec3 a_pos;

uniform mat4 u_proj;
uniform mat4 u_view;

void main() {
	gl_Position = u_proj * u_view * vec4(a_pos, 1.0);
	v_uv = a_pos;
}

