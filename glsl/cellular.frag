// From Patricio Gonzalez Vivo: https://thebookofshaders.com/12/

#ifdef GL_ES
precision mediump float;
#endif

uniform vec2 u_resolution;
uniform float u_time;

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

float cellularNoise(vec2 uv) {
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
            point = 0.5 + 0.5*sin(u_time + 6.2831*point);

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

void main()
{
    vec2 uv = gl_FragCoord.xy / u_resolution.xy;
    uv.x *= u_resolution.x / u_resolution.y;

    float scale = 15.;
    uv *= scale;

    vec3 noise = vec3(cellularNoise(uv));

    vec3 base_color = vec3(0., 0., 0.);
    noise += base_color;

    gl_FragColor = vec4(noise, 1.0);
}
