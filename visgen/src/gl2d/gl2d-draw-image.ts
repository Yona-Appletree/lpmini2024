import { Gl2dContext } from "./gl2d-context.ts";

export function gl2dDrawImage(context: Gl2dContext, image: CanvasImageSource) {
  const { gl, framebuffers, vertexShader } = context;
  gl.bindFramebuffer(gl.FRAMEBUFFER, framebuffers[0].framebuffer);
  gl.clearColor(0, 0, 0, 1);
  gl.clear(gl.COLOR_BUFFER_BIT);

  const texture = gl.createTexture();
  gl.bindTexture(gl.TEXTURE_2D, texture);
  gl.texImage2D(
    gl.TEXTURE_2D,
    0,
    gl.RGBA,
    gl.RGBA,
    gl.UNSIGNED_BYTE,
    image as TexImageSource,
  );
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.LINEAR);
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.LINEAR);
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);

  // Create a simple program to draw the image
  const fragmentShader = gl.createShader(gl.FRAGMENT_SHADER)!;
  gl.shaderSource(
    fragmentShader,
    `
      precision highp float;
      in vec2 vUv;
      uniform sampler2D uTexture;
      out vec4 fragColor;
      void main() {
        fragColor = texture(uTexture, vUv);
      }
    `,
  );
  gl.compileShader(fragmentShader);

  // Check fragment shader compilation
  if (!gl.getShaderParameter(fragmentShader, gl.COMPILE_STATUS)) {
    console.error(
      "Fragment shader compilation error:",
      gl.getShaderInfoLog(fragmentShader),
    );
    gl.deleteShader(fragmentShader);
    throw new Error("Failed to compile fragment shader");
  }

  const program = gl.createProgram()!;
  gl.attachShader(program, vertexShader);
  gl.attachShader(program, fragmentShader);
  gl.linkProgram(program);

  // Check program linking
  if (!gl.getProgramParameter(program, gl.LINK_STATUS)) {
    console.error("Program linking error:", gl.getProgramInfoLog(program));
    gl.deleteProgram(program);
    gl.deleteShader(fragmentShader);
    throw new Error("Failed to link program");
  }

  gl.useProgram(program);

  const positionLocation = gl.getAttribLocation(program, "aVertexPosition");
  gl.enableVertexAttribArray(positionLocation);
  gl.vertexAttribPointer(positionLocation, 2, gl.FLOAT, false, 0, 0);

  gl.drawArrays(gl.TRIANGLE_STRIP, 0, 4);

  // Clean up
  gl.deleteProgram(program);
  gl.deleteShader(fragmentShader);
}
