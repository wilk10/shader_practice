#version 450

layout(location = 0) in vec2 uv;
// layout(location = 1) in vec4 position;
layout(location = 0) out vec4 o_Target;

layout(set = 2, binding = 0) uniform texture2DArray FireTexture_texture;
layout(set = 2, binding = 1) uniform sampler FireTexture_texture_sampler;

layout(set = 3, binding = 0) uniform FireMaterial {
    vec4 base_color;
    float power;
    float detail_level;
    float bottom_threshold;
};

layout(set = 4, binding = 0) uniform TimeComponent_value {
    float u_time;
};

precision mediump float;

//////////////////////////
// Classic Perlin Noise
//
// From Patricio Gonzalez Vivo: 
// https://gist.github.com/patriciogonzalezvivo/670c22f3966e662d2f83
//
// and also:
// https://stackoverflow.com/questions/21272465/glsl-shadows-with-perlin-noise 
//

vec4 mod289(vec4 x) {
    return x - floor(x * (1.0 / 289.0)) * 289.0;
}

vec4 permute(vec4 x) {
    return mod289(((x*34.0)+1.0)*x);
}

vec2 fade(vec2 t) {return t*t*t*(t*(t*6.0-15.0)+10.0);}

float classicPerlinNoise(vec2 P, float perlin_scale_factor) {
    // scale
    P *= perlin_scale_factor;

    vec4 Pi = floor(P.xyxy) + vec4(0.0, 0.0, 1.0, 1.0);
    vec4 Pf = fract(P.xyxy) - vec4(0.0, 0.0, 1.0, 1.0);
    Pi = mod(Pi, 289.0); // To avoid truncation effects in permutation
    vec4 ix = Pi.xzxz;
    vec4 iy = Pi.yyww;
    vec4 fx = Pf.xzxz;
    vec4 fy = Pf.yyww;
    vec4 i = permute(permute(ix) + iy);
    vec4 gx = 2.0 * fract(i * 0.0243902439) - 1.0; // 1/41 = 0.024...
    vec4 gy = abs(gx) - 0.5;
    vec4 tx = floor(gx + 0.5);
    gx = gx - tx;
    vec2 g00 = vec2(gx.x,gy.x);
    vec2 g10 = vec2(gx.y,gy.y);
    vec2 g01 = vec2(gx.z,gy.z);
    vec2 g11 = vec2(gx.w,gy.w);
    vec4 norm = 1.79284291400159 - 0.85373472095314 * 
    vec4(dot(g00, g00), dot(g01, g01), dot(g10, g10), dot(g11, g11));
    g00 *= norm.x;
    g01 *= norm.y;
    g10 *= norm.z;
    g11 *= norm.w;
    float n00 = dot(g00, vec2(fx.x, fy.x));
    float n10 = dot(g10, vec2(fx.y, fy.y));
    float n01 = dot(g01, vec2(fx.z, fy.z));
    float n11 = dot(g11, vec2(fx.w, fy.w));
    vec2 fade_xy = fade(Pf.xy);
    vec2 n_x = mix(vec2(n00, n01), vec2(n10, n11), fade_xy.x);
    float n_xy = mix(n_x.x, n_x.y, fade_xy.y);
    return 2.3 * n_xy;
}
//////////////////////////

//////////////////////////
// Cellular Noise
// from Patricio Gonzalez Vivo: https://thebookofshaders.com/12/
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

			// Animate the point
            // point = 0.5 + 0.5 * sin(u_time + 6.2831 * point);

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

void main()
{
    // float cellular_scale_factor = 15.0;
    // float perlin_scale_factor = 8.0;

    // float animated_cellular = cellularNoise(uv, cellular_scale_factor);
    // float classic_perlin = classicPerlinNoise(uv, perlin_scale_factor);
    // float animated_classic_perlin = 0.5 + 0.5 * sin(u_time + 6.2831 * classic_perlin);

    // float total_noise = animated_classic_perlin * animated_cellular;
    // vec2 vertical_lerped_uv = mix(uv, vec2(total_noise), vec2(0., 1.));

    vec2 vertical_lerped_uv = uv; 

    vec4 result = texture(sampler2DArray(FireTexture_texture, FireTexture_texture_sampler), vec3(vertical_lerped_uv, 0.));
    result += texture(sampler2DArray(FireTexture_texture, FireTexture_texture_sampler), vec3(vertical_lerped_uv, 1.));
    result += texture(sampler2DArray(FireTexture_texture, FireTexture_texture_sampler), vec3(vertical_lerped_uv, 2.));
    result += base_color;
    o_Target = result;
}
