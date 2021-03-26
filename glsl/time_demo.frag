#ifdef GL_ES
precision mediump float;
#endif

uniform vec2 u_resolution;
uniform float u_time;

void main() {
    vec2 uv = gl_FragCoord.xy / u_resolution;
    float translation = sin(u_time / 2.0);
    float threshold = uv.x + translation * 0.5;

    vec3 red = vec3(1.,0.,0.);
    vec3 blue = vec3(0.,0.,1.);
    vec3 mixed_colors = mix(red, blue, threshold);

    gl_FragColor = vec4(mixed_colors, 1.0);
}
