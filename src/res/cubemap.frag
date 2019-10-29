// wengwengweng

varying vec3 v_uv;

uniform vec4 u_color;
uniform samplerCube u_tex;

void main() {
	gl_FragColor = textureCube(u_tex, v_uv) * u_color;
}

