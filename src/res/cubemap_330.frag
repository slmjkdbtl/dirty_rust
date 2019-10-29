// wengwengweng

varying vec3 v_uv;

uniform samplerCube u_tex;

out vec4 o_color;

void main() {
	o_color = texture(u_tex, v_uv);
}

