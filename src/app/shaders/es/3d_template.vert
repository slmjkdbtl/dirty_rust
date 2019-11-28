#version 300 es

in vec3 a_pos;
in vec3 a_normal;
in vec2 a_uv;
in vec4 a_color;

out vec3 v_pos;
out vec3 v_normal;
out vec2 v_uv;
out vec4 v_color;

uniform mat4 u_model;
uniform mat4 u_view;
uniform mat4 u_proj;

###REPLACE###

void main() {

	v_pos = (u_model * vec4(a_pos, 1.0)).xyz;
	v_uv = a_uv;
	v_color = a_color;
	v_normal = normalize((u_model * vec4(a_normal, 1.0)).xyz);
	gl_Position = vert();

}

