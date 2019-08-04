// wengwengweng

// varying vec2 v_uv;
in vec4 v_color;
in float v_brightness;

out vec4 frag_color;

uniform sampler2D tex;

###REPLACE###

void main() {
	frag_color = vec4(v_color.xzy * v_brightness, 1);
}

