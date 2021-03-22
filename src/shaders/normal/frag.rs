pub const FRAGMENT_SHADER: &str = r#"
#version 450
layout(location = 0) in vec3 normal;
layout(location = 0) out vec4 o_Target;

void main() {
    o_Target = vec4(normal, 1.0);
}
"#;
