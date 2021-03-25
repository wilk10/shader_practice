#ifdef GL_ES
precision mediump float;
#endif

uniform vec2 u_resolution;
uniform float u_time;

void main() {
    vec2 uv = gl_FragCoord.xy / u_resolution.xy;
    uv.x *= u_resolution.x / u_resolution.y;
    // uv = vec2(uv.x, 1. - uv.y);

    float bottom_threshold = -0.5;
    vec2 squared_uv = sqrt(uv);
    float adjusted_bottom = 2. * (1. - squared_uv.y + bottom_threshold);
    float clamped_bottom = clamp(adjusted_bottom, 0., 1.);
    vec3 result = vec3(1. - clamped_bottom);

    gl_FragColor = vec4(result, 1.0);
}
