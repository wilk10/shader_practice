#version 450

layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 o_Target;

layout(set = 2, binding = 0) uniform TimeComponent_value {
    float time;
};

precision mediump float;

void main() {
    float speed = 0.7;
    float translation = sin(time * speed);
    float percentage_extent = 0.6;
    float threshold = uv.x + translation * percentage_extent;

    vec3 red = vec3(1.,0.,0.);
    vec3 blue = vec3(0.,0.,1.);
    vec3 mixed_colors = mix(red, blue, threshold);

    o_Target = vec4(mixed_colors, 1.0);
}
