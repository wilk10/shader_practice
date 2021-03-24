#ifdef GL_ES
precision mediump float;
#endif

uniform vec2 u_resolution;

float rectShape(vec2 position, vec2 dim) {
    float scalingFactor = 0.5;
    dim = vec2(scalingFactor) - dim * scalingFactor;
    vec2 shaper = vec2(step(dim.x, position.x), step(dim.y, position.y));
    shaper *= vec2(step(dim.x, 1.0 - position.x), step(dim.y, 1.0 - position.y));
    return shaper.x * shaper.y;
}

void main() {
    vec2 position = gl_FragCoord.xy / u_resolution;
    float width = 0.3;
    float height = 0.5;
    vec2 dimensions = vec2(width, height);
    float rectangle = rectShape(position, dimensions);
    vec3 colour = vec3(0.0);
    colour = vec3(rectangle);
    gl_FragColor = vec4(colour, 1.0);
}
