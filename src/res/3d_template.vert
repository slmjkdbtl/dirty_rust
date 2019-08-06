// wengwengweng

attribute vec3 pos;
attribute vec3 normal;
attribute vec4 color;

// varying vec2 v_uv;
varying vec4 v_color;

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

