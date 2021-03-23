#version 450

layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 o_Target;

layout(set = 2, binding = 0) uniform SpawnVfx {
    vec4 color_a;
    vec4 color_b;
    float start_lerp;
    float end_lerp;
};

layout(set = 3, binding = 0) uniform TimeComponent_value {
    float time;
};

void main()
{
    float tau = 6.283185307179586;
    
    float horizontal_repetitions = 8.0;
    float dampen_factor = 0.01;
    float offset = cos(uv.x * tau * horizontal_repetitions) * dampen_factor;
    float time_adjustment = 0.2;
    float wiggle = uv.y + offset + time * time_adjustment;

    float vertical_repetitions = 5.0;
    float pattern_minus1to1 = cos(wiggle * tau * vertical_repetitions);
    float pattern_from0to1 = pattern_minus1to1 * 0.5 + 0.5;

    float fade = pattern_from0to1 * uv.y;
    vec4 effect = mix(color_a, color_b, fade);
    o_Target = effect;
}
