// wengwengweng

#version 330 core

layout(location = 0) in vec2 vertices;
layout(location = 1) in vec2 uv;

out vec2 tex_coord;
uniform mat4 trans;
uniform mat4 proj;
uniform vec4 quad;

void main() {
	gl_Position = vec4(vertices, 0.0, 1.0);
}

