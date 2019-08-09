// wengwengweng
// from http://glslsandbox.com/e#56572.0

uniform float time;
uniform vec2 mouse;
uniform vec2 resolution;

float PI = acos(-1.);

float fold(float x) {
	float span = -abs(sin(time));
	return x / span < 5.0 ? mod(x, span) - span * .5 : x;
}

vec3 trans(vec3 p) {
	return vec3(
		fold(p.x), fold(p.y), fold(p.z)
	);
}

float makeSphere(vec3 p) {
	return length(trans(p)) - 1.0;
}

vec4 frag(sampler2D tex, vec2 uv) {

	vec2 p = (gl_FragCoord.xy * 2. - resolution) / min(resolution.x, resolution.y);

	vec3 cam_pos = vec3(sin(mouse.y * PI) * 2., cos(mouse.x * PI) * 2. , sin(time) * 2.);
	vec3 cam_up = vec3(sin(time), cos(time),0.);
	vec3 cam_dir = normalize(-cam_pos);
	vec3 cam_side = cross(cam_dir, cam_up);

	vec3 ray = normalize(cam_side * p.x + cam_up * p.y + cam_dir);


	vec3 d = vec3(p, 0.4);

	vec3 ray_pos = cam_pos;
	float ray_length = 0.;
	float inter = 0.;

	for (int i = 0; i < 10; i++) {

		inter = makeSphere(ray_pos);
		ray_length += inter;
		ray_pos = cam_pos + ray * ray_length;

		d.yzx = normalize(abs(ray_pos)) * (abs(d) / dot(d,d) - (normalize(abs(inter * ray_pos)) * .1));

	}

	vec3 color = vec3(.0);

	if (inter < 0.001) {
		color = d;
	}

	return vec4(color, 1.0);

}

