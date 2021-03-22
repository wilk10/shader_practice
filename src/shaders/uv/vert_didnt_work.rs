// from: https://www.khronos.org/opengl/wiki/Compute_eye_space_from_window_space

pub const VERTEX_SHADER: &str = r#"
#version 450
layout(location = 0) in vec3 Vertex_Position;
layout(location = 1) in vec2 Vertex_Uv;
layout(location = 0) out vec2 uv;
layout(location = 1) out vec3 eyeDirection;

layout(set = 0, binding = 0) uniform Camera {
    mat4 ViewProj;
};
layout(set = 1, binding = 0) uniform Transform {
    mat4 Model;
};
layout(set = 2, binding = 0) uniform Plane {
    vec2 halfSizeNearPlane; 
};

void main() {
    gl_Position = ViewProj * Model * vec4(Vertex_Position, 1.0);
    uv = Vertex_Uv;
    eye_direction = vec3((2.0 * halfSizeNearPlane * texCoord) - halfSizeNearPlane , -1.0);
}
"#;
