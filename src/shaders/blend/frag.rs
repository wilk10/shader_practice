pub const FRAGMENT_SHADER: &str = r#"
#version 450
layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 o_Target;
layout(set = 2, binding = 0) uniform BlendColors_color_a {
    vec4 color_a;
};
layout(set = 2, binding = 1) uniform BlendColors_color_b {
    vec4 color_b;
};
layout(set = 2, binding = 2) uniform BlendColors_start_lerp {
    float start_lerp;
};
layout(set = 2, binding = 3) uniform BlendColors_end_lerp {
    float end_lerp;
};

precision mediump float;

float inverseLerp(float from, float to, float value) {
    return (value - from) / (to - from);
}

void main()
{
    float inv_lerped_uvx = inverseLerp(start_lerp, end_lerp, uv.x);
    float min_value = 0.0;
    float max_value = 1.0;
    float gradient = clamp(inv_lerped_uvx, min_value, max_value);
    o_Target = mix(color_a, color_b, gradient);
}
"#;
