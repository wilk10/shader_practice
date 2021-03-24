#ifdef GL_ES
precision mediump float;
#endif

uniform vec2 u_resolution;
uniform float u_time;

float circleShape(vec2 position, float radius) {
    return step(radius, length(position - vec2(0.5)));
}

void main() {
    vec2 position = gl_FragCoord.xy / u_resolution;
    vec2 translation = vec2(sin(u_time / 2.0), 0.0);
    position += translation * 0.5;
    float circle = circleShape(position, 0.25);
    vec3 colour = vec3(circle);
    gl_FragColor = vec4(colour, 1.0);
}
