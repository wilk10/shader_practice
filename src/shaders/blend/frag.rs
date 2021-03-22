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
layout(set = 2, binding = 2) uniform BlendColors_color_start {
    float color_start;
};
layout(set = 2, binding = 3) uniform BlendColors_color_end {
    float color_end;
};

float inverseLerp(float from, float to, float value) {
    return (value - from) / (to - from);
}

void main()
{
    float inv_lerped_uvx = inverseLerp(color_start, color_end, uv.x);
    float gradient = clamp(inv_lerped_uvx, 0.0, 1.0);
    o_Target = mix(color_a, color_b, gradient);
}
"#;
