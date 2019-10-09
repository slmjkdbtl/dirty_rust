// wengwengweng

#version 330

layout(location = 0) in vec3 a_pos;
layout(location = 1) in vec2 a_uv;
layout(location = 2) in vec3 a_normal;
layout(location = 3) in vec4 a_color;

uniform mat4 u_model;
uniform mat4 u_view;
uniform mat4 u_proj;

// out vec2 v_uv;
out vec4 v_color;

###REPLACE###

void main() {

	vec3 world_pos = (u_model * vec4(a_pos, 1.0)).xyz;

// 	v_uv = uv;
	v_color = a_color;
	gl_Position = u_proj * u_view * vec4(world_pos, 1.0);

}

