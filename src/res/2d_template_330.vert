// wengwengweng

#version 330

layout(location = 0) in vec2 pos;
layout(location = 1) in vec2 uv;
layout(location = 2) in vec4 color;

out vec2 v_uv;
out vec4 v_color;

uniform mat4 proj;

###REPLACE###

void main() {

	v_color = color;
	v_uv = uv;
	gl_Position = vert(proj, vec4(pos, 1.0, 1.0));

}

