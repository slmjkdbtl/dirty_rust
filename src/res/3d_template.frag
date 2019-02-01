// wengwengweng

varying vec4 tint;

vec4 effect(vec4 color) {
	return color;
}

void main() {
	gl_FragColor = effect(tint);
}

