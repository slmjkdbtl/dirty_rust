// wengwengweng

attribute vec3 a_pos;
attribute vec3 a_normal;
attribute vec2 a_uv;
attribute vec4 a_color;

varying vec2 v_uv;
varying float v_brightness;
varying vec4 v_color;

uniform mat4 u_model;
uniform mat4 u_view;
uniform mat4 u_proj;

###REPLACE###

void main() {

	vec3 world_pos = (u_model * vec4(a_pos, 1.0)).xyz;

// 	vec3 unit_normal = normalize((u_model * vec4(a_normal, 1.0)).xyz);
// 	vec3 unit_light_dir = normalize(vec3(0, 240, 0) - world_pos);

// 	v_brightness = max(dot(unit_normal, unit_light_dir), 0.05);
	v_uv = a_uv;
	v_color = a_color;
	gl_Position = u_proj * u_view * vec4(world_pos, 1.0);

}

