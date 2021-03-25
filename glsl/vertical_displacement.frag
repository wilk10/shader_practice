#ifdef GL_ES
precision mediump float;
#endif

uniform vec2 u_resolution;
uniform float u_time;

vec2 animateVertically(vec2 input_uv, float time, float factor) {
    vec2 vertical_animation = vec2(0., sin(factor * time));
    vec2 displace_vertically = input_uv + vertical_animation;
    // vec2 clamp_displacement = clamp(displace_vertically, 0., 1.);
    return displace_vertically;
}

void main() {
    vec2 uv = gl_FragCoord.xy / u_resolution.xy;
    // uv = vec2(uv.x, 1. - uv.y);

    // vec2 result = uv;
    vec2 result = animateVertically(uv, u_time, 0.5);
    // result = clamp(result, 0., 1.);

    gl_FragColor = vec4(result, 0.0, 1.0);
}
