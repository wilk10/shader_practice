#ifdef GL_ES
precision mediump float;
#endif

uniform vec2 u_resolution;

float circleShape(vec2 position, float radius) {
    return step(radius, length(position - vec2(0.5)));
}

void main() {
    vec2 position = gl_FragCoord.xy / u_resolution;
    float circle = circleShape(position, 0.2);
    vec3 colour = vec3(circle);
    gl_FragColor = vec4(colour, 1.0);
}
