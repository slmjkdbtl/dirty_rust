// wengwengweng

uniform vec3 u_cam_pos;
uniform float u_fog_level;
uniform vec4 u_fog_color;

vec4 frag() {

	// init
	vec4 c = default_color();

	// normal
	float l = (v_normal.x + v_normal.y + v_normal.z) / 3.0;
	c = mix(c, vec4(vec3(l), 1.0), 0.24);

	// fog
	float dis = distance(u_cam_pos, v_pos);
	c = mix(c, u_fog_color, dis * u_fog_level * 0.01);

	return c;

}

