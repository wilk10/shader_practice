#version 450

layout(location = 0) in vec2 uv;
layout(location = 1) in vec4 position;
layout(location = 0) out vec4 o_Target;

layout(set = 2, binding = 0) uniform FireMaterial {
    vec4 base_color;
    float power;
    float detail_level;
    float bottom_threshold;
};

layout(set = 3, binding = 0) uniform texture2DArray FireTexture_texture;
layout(set = 3, binding = 1) uniform sampler FireTexture_texture_sampler;

precision mediump float;

// float inverseLerp(float from, float to, float value) {
//     return (value - from) / (to - from);
// }

// float rand2dTo1d(vec2 value, vec2 dotDir){
// 	vec2 smallValue = sin(value);
// 	float random = dot(smallValue, dotDir);
// 	random = fract(sin(random) * 143758.5453);
// 	return random;
// }

float rand2dTo1d(vec2 uv, float offset, vec2 dotDir) {
	vec2 smallValue = sin(uv);
	float random = dot(smallValue, dotDir);
    random = (random * offset) * 0.5 + 0.5;
	random = fract(sin(random) * 143758.5453);
	return random;
}

vec2 rand2dTo2dOffset(vec2 uv, float offset) {
	return vec2(
		rand2dTo1d(uv, offset, vec2(12.989, 78.233)),
		rand2dTo1d(uv, offset, vec2(39.346, 11.135))
	);
}

vec2 voronoiNoise(vec2 uv, float angleOffset, float cellDensity) {
    vec2 baseCell = floor(uv * cellDensity);
    vec2 fraction = fract(uv * cellDensity);
    float minDistToCell = 8.0;
    vec2 result = vec2(minDistToCell, 0.0);

    for(int x=-1; x<=1; x++) {
        for(int y=-1; y<=1; y++) {
            vec2 lattice = vec2(x, y);
            vec2 cell = baseCell + vec2(x, y);
            vec2 offset = rand2dTo2dOffset(cell, angleOffset);
            float dist = distance(lattice + offset, fraction);
            if (dist < minDistToCell) {
                result = vec2(dist, offset.x);
            }
        }
    }
    return result;
}

void main()
{
    // float inv_lerped_uvx = inverseLerp(start_lerp, end_lerp, uv.x);
    // float min_value = 0.0;
    // float max_value = 1.0;
    // float gradient = clamp(inv_lerped_uvx, min_value, max_value);
    // o_Target = mix(color_a, color_b, gradient);

    // vec4 output = texture(sampler2DArray(FireTexture_texture, FireTexture_texture_sampler), vec3(uv, 0.));
    // output += texture(sampler2DArray(FireTexture_texture, FireTexture_texture_sampler), vec3(uv, 1.));
    // output += texture(sampler2DArray(FireTexture_texture, FireTexture_texture_sampler), vec3(uv, 2.));
    // output += base_color;

    // ? distortion = noise1d(?) * voronoiNoise(?);

    float angle_offset = 2.0;
    float cell_density = 100.0;

    vec4 result = vec4(voronoiNoise(uv, angle_offset, cell_density), 0., 1.);
    result += base_color;
    o_Target = result;
}
