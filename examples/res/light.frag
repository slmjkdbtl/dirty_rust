// wengwengweng

struct Light {
	vec3 pos;
	vec3 color;
};

struct Material {
	float diffuse;
	float specular;
	float shininess;
};

uniform mat4 u_model;
uniform Light u_light;
uniform Material u_material;
uniform vec3 u_cam_pos;

vec4 frag() {

	vec4 obj_color = v_color * u_color * texture2D(u_tex, v_uv);
	vec3 normal = normalize((u_model * vec4(v_normal, 1.0)).xyz);

	vec3 w_pos = (u_model * vec4(v_pos, 1.0)).xyz;

	vec3 light_color = vec3(0);
	vec3 light_dir = normalize(u_light.pos - w_pos);

	// diffuse
	float df = max(dot(normal, light_dir), 0.0);
	vec3 df_color = mix(vec3(1), df * u_light.color, u_material.diffuse);

	// specular
	vec3 view_dir = normalize(u_cam_pos - w_pos);
	vec3 reflect_dir = reflect(-light_dir, normal);
	float spec = pow(max(dot(view_dir, reflect_dir), 0.0), u_material.shininess);
	vec3 sp_color = u_material.specular * spec * u_light.color;

	light_color += (df_color + sp_color);

	return vec4(obj_color.rgb * light_color, obj_color.a);

}

