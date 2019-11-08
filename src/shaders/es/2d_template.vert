#version 300 es

in vec3 a_pos;
in vec2 a_uv;
in vec4 a_color;

out vec2 v_uv;
out vec4 v_color;

uniform mat4 u_proj;

###REPLACE###

void main() {

	v_color = a_color;
	v_uv = a_uv;
	gl_Position = vert();

}

// const vec2 verts[3] = vec2[3](
// 	vec2(0.5f, 1.0f),
// 	vec2(0.0f, 0.0f),
// 	vec2(1.0f, 0.0f)
// );
// out vec2 vert;
// void main() {
// 	vert = verts[gl_VertexID];
// 	gl_Position = vec4(vert - 0.5, 0.0, 1.0);
// }
