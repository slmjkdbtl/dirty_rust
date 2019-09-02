// wengwengweng

attribute vec3 pos;
attribute vec3 normal;
attribute vec2 uv;
attribute vec4 color;

varying vec2 v_uv;
varying float v_brightness;
varying vec4 v_color;

uniform mat4 model;
uniform mat4 view;
uniform mat4 proj;

###REPLACE###

void main() {

	vec3 world_pos = (model * vec4(pos, 1.0)).xyz;

// 	vec3 unit_normal = normalize((model * vec4(normal, 1.0)).xyz);
// 	vec3 unit_light_dir = normalize(vec3(0, 240, 0) - world_pos);

// 	v_brightness = max(dot(unit_normal, unit_light_dir), 0.05);
	v_uv = uv;
	v_color = color;
	gl_Position = proj * view * vec4(world_pos, 1.0);

}

