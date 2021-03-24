#ifdef GL_ES
precision mediump float;
#endif

const float PI = 3.1415926535;

uniform vec2 u_resolution;

float polygonShape(vec2 position, float radius, float nSides) {
    position = position * 2.0 - 1.0;
    float angle = atan(position.x, position.y);
    float slice = PI * 2.0 / nSides;
    return step(radius, cos(floor(0.5 + angle / slice) * slice - angle) * length(position));
}

void main() {
    vec2 position = gl_FragCoord.xy / u_resolution;
    float polygon = polygonShape(position, 0.5, 7.0);
    vec3 colour = vec3(0.0);
    colour = vec3(polygon);
    gl_FragColor = vec4(colour, 1.0);
}
