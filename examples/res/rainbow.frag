// wengwengweng

uniform float u_time;
uniform float u_size;

float hue2rgb(float f1, float f2, float hue) {

	if (hue < 0.0) {
		hue += 1.0;
	} else if (hue > 1.0) {
		hue -= 1.0;
	}

	float res;

	if ((6.0 * hue) < 1.0) {
		res = f1 + (f2 - f1) * 6.0 * hue;
	} else if ((2.0 * hue) < 1.0) {
		res = f2;
	} else if ((3.0 * hue) < 2.0) {
		res = f1 + (f2 - f1) * ((2.0 / 3.0) - hue) * 6.0;
	} else {
		res = f1;
	}

	return res;

}

vec3 hsl2rgb(vec3 hsl) {

	vec3 rgb;

	if (hsl.y == 0.0) {

		rgb = vec3(hsl.z);

	} else {

		float f2;

		if (hsl.z < 0.5) {
			f2 = hsl.z * (1.0 + hsl.y);
		} else {
			f2 = hsl.z + hsl.y - hsl.y * hsl.z;
		}

		float f1 = 2.0 * hsl.z - f2;

		rgb.r = hue2rgb(f1, f2, hsl.x + (1.0/3.0));
		rgb.g = hue2rgb(f1, f2, hsl.x);
		rgb.b = hue2rgb(f1, f2, hsl.x - (1.0/3.0));

	}

	return rgb;

}

vec3 hsl2rgb(float h, float s, float l) {
	return hsl2rgb(vec3(h, s, l));
}

vec4 frag() {

	float pos = v_pos.x / u_size + v_pos.y / u_size + v_pos.z / u_size;
	float h = mod(pos + u_time, 1.0);
	float s = 1.0;
	float l = 0.5 + pow(sin(v_pos.x * 3.0 + v_pos.y * 0.5 + u_time), 2.0) * 0.3;
	float n = (v_normal.x + v_normal.y + v_normal.z) / 3.0;
	vec4 c = vec4(hsl2rgb(h,s,l), 1);
	c = mix(c, vec4(vec3(n), 1.0), 0.4);

	return c;

}

