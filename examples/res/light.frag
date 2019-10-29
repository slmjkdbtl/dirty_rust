// wengwengweng

uniform vec3 u_light_pos;
uniform vec3 u_light_color;
uniform float u_light_mix;

vec4 frag() {

	vec3 normal = normalize(v_normal);
	vec3 light_dir = normalize(u_light_pos - v_pos);
	float diff = max(dot(normal, light_dir), 0.0);
	vec3 diffuse = diff * u_light_color;

	return mix(v_color * texture2D(u_tex, v_uv), vec4(diffuse, 1.0), u_light_mix);

}

