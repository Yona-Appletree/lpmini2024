// Gaussian blur shader with 5x5 kernel
export const blurShader = `
      #version 300 es
      precision highp float;
      
      in vec2 vUv;
      out vec4 fragColor;
      uniform vec2 uResolution;
      uniform sampler2D uInputTexture;

      void main() {
        vec2 pixelSize = 1.0 / uResolution;
        vec4 color = vec4(0.0);
        
        // Gaussian kernel weights
        float weights[5] = float[5](0.227027, 0.316216, 0.070270, 0.008491, 0.000000);
        
        // Horizontal blur
        for (int i = -2; i <= 2; i++) {
          vec2 offset = vec2(float(i)) * pixelSize;
          color += texture(uInputTexture, vUv + offset) * weights[abs(i)];
        }
        
        // Vertical blur
        vec4 finalColor = vec4(0.0);
        for (int i = -2; i <= 2; i++) {
          vec2 offset = vec2(0.0, float(i)) * pixelSize;
          finalColor += texture(uInputTexture, vUv + offset) * weights[abs(i)];
        }
        
        fragColor = finalColor;
      }
    `.trim();
