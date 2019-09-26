// wengwengweng

#version 330

layout(location = 0) in vec3 pos;
layout(location = 1) in vec2 uv;
layout(location = 2) in vec3 normal;
layout(location = 3) in vec4 color;

// out vec2 v_uv;
out vec4 v_color;

uniform mat4 model;
uniform mat4 view;
uniform mat4 proj;

###REPLACE###

void main() {

	vec3 world_pos = (model * vec4(pos, 1.0)).xyz;

// 	v_uv = uv;
	v_color = color;
	gl_Position = proj * view * vec4(world_pos, 1.0);

}

