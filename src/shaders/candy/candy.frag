#version 450

layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 o_Target;

layout(set = 2, binding = 0) uniform Candy {
    vec4 color_a;
    vec4 color_b;
    float start_lerp;
    float end_lerp;
};

float inverseLerp(float from, float to, float value) {
    return (value - from) / (to - from);
}

void main()
{
    float tau = 6.283185307179586;
    float lerped_coords = inverseLerp(start_lerp, end_lerp, (uv.x + uv.y));
    float repetitions = 5.0;
    float pattern_minus1to1 = cos(lerped_coords * tau * repetitions);
    float pattern_from0to1 = pattern_minus1to1 * 0.5 + 0.5;
    o_Target = mix(color_a, color_b, pattern_from0to1);
}
