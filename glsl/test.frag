#ifdef GL_ES
precision mediump float;
#endif

void main() {
    vec3 colour = vec3(1.0, 0.6, 0.3);

    gl_FragColor = vec4(colour, 1.0);
}
