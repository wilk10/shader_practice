#version 450

layout(location = 0) in vec2 _uv;
layout(location = 0) out vec4 o_Target;

layout(set = 2, binding = 0) uniform texture2DArray FireTexture_texture;
layout(set = 2, binding = 1) uniform sampler FireTexture_texture_sampler;

layout(set = 3, binding = 0) uniform FireMaterial {
    vec4 _base_color;
    float _flame_height;
    float _distorsion_level;
    float _bottom_threshold;
    float _time;
};

precision mediump float;

//////////////////////////
// Cellular Noise
//
// From:
// Patricio Gonzalez Vivo: https://thebookofshaders.com/12/
//
vec2 random2( vec2 p ) {
    return fract(
        sin(
            vec2(
                dot(p, vec2(127.1, 311.7)),
                dot(p, vec2(269.5, 183.3))
            )
        ) * 43758.5453
    );
}

float cellularNoise(vec2 uv, float scale_factor) {
    // Scale
    uv *= scale_factor;

    // Tile the space
    vec2 i_st = floor(uv);
    vec2 f_st = fract(uv);

    float m_dist = 1.;  // minimum distance

    for (int y= -1; y <= 1; y++) {
        for (int x= -1; x <= 1; x++) {
            // Neighbor place in the grid
            vec2 neighbor = vec2(float(x),float(y));

            // Random position from current + neighbor place in the grid
            vec2 point = random2(i_st + neighbor);

			// Vector between the pixel and the point
            vec2 diff = neighbor + point - f_st;

            // Distance to the point
            float dist = length(diff);

            // Keep the closer distance
            m_dist = min(m_dist, dist);
        }
    }
    return m_dist;
}
//////////////////////////

//////////////////////////
// Simple Noise 2D
//
// From:
// Patricio Gonzalez Vivo: https://thebookofshaders.com/12/
//
// And:
// <https://www.shadertoy.com/view/4dS3Wd>
// By Morgan McGuire @morgan3d, http://graphicscodex.com
//
float hash(vec2 p) { return fract(1e4 * sin(17.0 * p.x + p.y * 0.1) * (0.1 + abs(sin(p.y * 13.0 + p.x)))); }

float simpleNoise(vec2 uv, float scale_factor) {
    // Scale
    uv *= scale_factor;

	vec2 i = floor(uv);
	vec2 f = fract(uv);

	// Four corners in 2D of a tile
	float a = hash(i);
	float b = hash(i + vec2(1.0, 0.0));
	float c = hash(i + vec2(0.0, 1.0));
	float d = hash(i + vec2(1.0, 1.0));

	vec2 u = f * f * (3.0 - 2.0 * f);
	return mix(a, b, u.x) + (c - a) * u.y * (1.0 - u.x) + (d - b) * u.x * u.y;
}
//////////////////////////

vec2 animateVertically(vec2 uv, float time, float factor) {
    vec2 vertical_animation = vec2(0., factor * time);
    return uv + vertical_animation;
}

void main() {
    //// main cellular node
    // main cellular secondary simple noise node
    vec2 secondary_simple_noise_uv = animateVertically(_uv, _time, 0.5);
    float secondary_simple_noise = simpleNoise(secondary_simple_noise_uv, _distorsion_level);

    // main cellular lerp
    vec2 cellular_uv = animateVertically(_uv, _time, 0.25);
    vec2 lerped_cellular_uv = mix(cellular_uv, vec2(secondary_simple_noise), vec2(0.5));

    // final cellular calculation
    float cellular = cellularNoise(lerped_cellular_uv, _distorsion_level);
    ////

    //// main simple noise node
    vec2 main_simple_noise_uv = animateVertically(_uv, _time, 0.3);
    float main_simple_noise = simpleNoise(main_simple_noise_uv, _distorsion_level);

    //// combine noises
    float total_noise = main_simple_noise * cellular;
    vec2 vertical_lerped_uv = mix(_uv, vec2(total_noise), vec2(0., _flame_height));
    vertical_lerped_uv += vec2(0., _flame_height); // get the layers to overlap correctly 

    // iterate through layers and add noise to distort them
    int N_LAYERS = 3; 
    vec4 image = vec4(0.);
    for (int layer = 0; layer < N_LAYERS; layer++) {

        // add distortion to each texture layer
        vec3 distortion = vec3(vertical_lerped_uv, layer);
        image += texture(sampler2DArray(FireTexture_texture, FireTexture_texture_sampler), distortion);
    }

    //// adjust fire bottom
    // first flip them (because input uv are flipped vertically. possibly a bug, but not sure where)
    vec2 flipped_uv = vec2(_uv.x, 1. - _uv.y);
    
    // then adjust
    float adjusted_bottom = 2. * (1. - sqrt(flipped_uv).y + _bottom_threshold);
    float clamped_bottom = clamp(adjusted_bottom, 0., 1.);
    image -= vec4(clamped_bottom);
    ////

    //// add color and output
    vec4 result = clamp(image, vec4(0.), vec4(1.));
    result *= _base_color;
    result *= vec4(vec3(10.), 1.);
    o_Target = result;
}
