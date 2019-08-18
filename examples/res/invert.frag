// wengwengweng

uniform float radius;
uniform vec2 dir;
uniform vec2 resolution;

vec4 frag(sampler2D tex, vec2 uv) {

	vec4 c = texture2D(tex, uv);

	return vec4(vec3(1) - c.xyz, c.a);

}

