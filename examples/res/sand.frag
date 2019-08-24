#ifdef GL_ES
precision mediump float;
#endif

#extension GL_OES_standard_derivatives : enable

uniform float time;
uniform vec2 mouse;
uniform vec2 resolution;


const float PI = acos(-1.);
float PI2 = PI*2.;

mat2 rot(float a){
	float c = cos(a),s = sin(a);
	return mat2(c,s,-s,c);
}

vec2 pmod(vec2 p,float r){
	float a = atan(p.x,p.y)+PI/r;
	float n = PI2/r;
	a = floor(a/n)*n;
	return p*rot(-a);
}

float rand(float n){
    return fract(sin(n)*45943.321651);
}

float rand(vec2 p){
	return rand(dot(p.xy,vec2(31.3214,55.5432)));
}

float noise(vec2 p){
	vec2 i = floor(tan(p));
	vec2 f = fract(tan(p));

	vec2 u = f*f*(3.0-2.0*f);

	float a = rand(i);
	float b = rand(i + vec2(1.0,0.0));
	float c = rand(i + vec2(0.0,1.0));
	float d = rand(i + vec2(1.0,1.0));


	float n = mix(mix(mix(a,b,u.x),mix(c,d,u.x),u.y),(mix(c,d,u.x),mix(a,b,u.y),u.y),p.x*p.y);

	return n;

}

float sdbox(vec2 p){
	p=abs(p);
	float b =normalize(max(p.x,p.y*rand(sin(time)))-0.8);
	return b;
}

void main( void ) {

	vec2 p = ( gl_FragCoord.xy*2.0- resolution.xy ) /min(resolution.x,resolution.y);

	p*=rot(noise(p));
	p *= rot(time);
	p *= rot(noise(p));
	p =pmod(p,9.0);
	vec3 box = vec3(sdbox(p));

	for(int i= 0;i<15; i++){
		p.x += 0.9;
		p *= rot(noise(p)*time);
		vec3 box2 = vec3(sdbox(p));
		box = max(box,-box2);
	}



	vec3 c = vec3((sin(time)),cos(time),rand(rand(time)));

	vec3 col = vec3(noise(p));

	gl_FragColor = vec4(box,1.0);

}
