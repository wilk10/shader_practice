// from: https://www.khronos.org/opengl/wiki/Compute_eye_space_from_window_space

pub const FRAGMENT_SHADER: &str = r#"
#version 450
layout(location = 0) in vec2 uv;
layout(location = 1) in vec3 eyeDirection;
layout(location = 0) out vec4 o_Target;

layout(set = 0, binding = 0) uniform Matrix {
    mat4 persMatrix; 
};
layout(set = 1, binding = 0) uniform Depth {
    vec2 depthrange; 
};
layout(set = 2, binding = 0) uniform sampler2D depthTex;

vec4 CalcEyeFromWindow(in float windowZ, in vec3 eyeDirection)
{
  float ndcZ = (2.0 * windowZ - depthrange.x - depthrange.y) /
    (depthrange.y - depthrange.x);
  float eyeZ = persMatrix[3][2] / ((persMatrix[2][3] * ndcZ) - persMatrix[2][2]);
  return vec4(eyeDirection * eyeZ, 1.0);
}

void main()
{
    o_Target = CalcEyeFromWindow(texture(depthTex, uv), eyeDirection);
}
"#;
