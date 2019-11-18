// wengwengweng

#version 330

layout(location = 0) in vec3 a_pos;
layout(location = 1) in vec2 a_uv;
layout(location = 2) in vec3 a_normal;
layout(location = 3) in vec4 a_color;

uniform mat4 u_model;
uniform mat4 u_view;
uniform mat4 u_proj;

out vec3 v_pos;
out vec2 v_uv;
out vec4 v_color;
out vec3 v_normal;

###REPLACE###

void main() {

	v_pos = (u_model * vec4(a_pos, 1.0)).xyz;
	v_uv = a_uv;
	v_color = a_color;
	v_normal = normalize(a_normal);
	gl_Position = vert();

}

