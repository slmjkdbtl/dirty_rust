#ifdef GL_ES
precision mediump float;
#endif

#extension GL_OES_standard_derivatives : enable

uniform float time;
uniform vec2 mouse;
uniform vec2 resolution;
uniform vec2 surfaceSize;
varying vec2 surfacePosition;

mat2 ro( float a )
{
	float s = sin(a);
	float c = cos(a);
	return mat2( c, -s, s, c );
}

vec3 fn( vec2 p )
{
	float dp = dot(p,p);

	p /= dp;

	return fract( vec3( p, dp ) );
}

void main( void ) {

	vec2 p = surfacePosition;

	vec3 o = fn( (fn( p ).xy * 2.0 - 1.0) * ro(-time) * time );

	gl_FragColor = vec4( o, 1.0 );

}


