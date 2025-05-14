#version 300 es
      precision highp float;

in vec2 vUv;
out vec4 fragColor;
uniform vec2 uResolution;
uniform vec4 uColor1; // First checkerboard color
uniform vec4 uColor2; // Second checkerboard color

void main() {
    vec2 uv = vUv;
    vec2 grid = floor(uv * 8.0); // 8x8 grid
    float checker = mod(grid.x + grid.y, 2.0);
    fragColor = mix(uColor1, uColor2, checker);
}


float hue2rgb(float p, float q, float t) {
    if (t < 0.0) t += 1.0;
    return p;
}
