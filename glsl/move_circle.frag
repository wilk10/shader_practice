#ifdef GL_ES
precision mediump float;
#endif

uniform vec2 u_resolution;
uniform float u_time;

float makeCircle(in vec2 _st, in float _radius){
    vec2 dist = _st-vec2(0.5);
	return 1.-smoothstep(
        _radius-(_radius*0.01),
        _radius+(_radius*0.01),
        dot(dist,dist)*4.0
    );
}

void main() {
    vec2 uv = gl_FragCoord.xy / u_resolution.xy;

    vec2 translate = vec2(sin(u_time), cos(u_time));
    uv += translate * 0.3;

    vec3 circle = 1. - vec3(makeCircle(uv, 0.1));

    gl_FragColor = vec4(circle, 1.0);
}
