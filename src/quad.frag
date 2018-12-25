// wengwengweng

#version 330 core

uniform sampler2D tex;
uniform vec4 tint;
in vec2 tex_coord;
out vec4 frag_color;

void main() {
	frag_color = texture(tex, tex_coord) * tint;
}

// #version 100
// precision mediump float;

// uniform sampler2D tex;
// varying vec2 tex_coord;
// uniform vec4 tint;

// void main() {
// 	gl_FragColor = texture2D(tex, tex_coord) * tint;
// }


