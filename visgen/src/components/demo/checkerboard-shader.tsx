// Checkerboard shader
export const checkerboardShader = `
      #version 300 es
      precision highp float;
      
      in vec2 vUv;
      out vec4 fragColor;
      uniform vec2 uResolution;

      void main() {
        vec2 uv = vUv;
        vec2 grid = floor(uv * 8.0); // 8x8 grid
        float checker = mod(grid.x + grid.y, 2.0);
        fragColor = vec4(vec3(checker), 1.0);
      }
    `.trim();
