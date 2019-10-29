// wengwengweng

uniform vec3 u_light_pos;
uniform vec3 u_light_color;
uniform float u_light_mix;

vec4 frag() {

	vec4 obj_color = v_color * texture2D(u_tex, v_uv);
	vec3 normal = normalize(v_normal);
	vec3 light_dir = normalize(u_light_pos - v_pos);
	float df = max(dot(normal, light_dir), 0.0);
	vec3 df_color = mix(vec3(1), df * u_light_color, u_light_mix);

	return vec4(obj_color.rgb * df_color, obj_color.a);

}

