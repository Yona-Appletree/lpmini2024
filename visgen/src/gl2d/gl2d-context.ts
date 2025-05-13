import { createCanvas } from "../util/create-canvas.ts";

export function Gl2dContext(canvas = createCanvas()) {
  const gl = canvas.getContext("webgl2")!;
  const width = canvas.width;
  const height = canvas.height;

  // Create vertex shader
  const vertexShaderSource = `
    #version 300 es
    in vec4 aVertexPosition;
    out vec2 vUv;
    void main() {
      vUv = (aVertexPosition.xy + 1.0) * 0.5;
      gl_Position = aVertexPosition;
    }
  `.trim();

  // Create position buffer
  const positions = new Float32Array([
    -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, 1.0,
  ]);
  const positionBuffer = gl.createBuffer();
  gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer);
  gl.bufferData(gl.ARRAY_BUFFER, positions, gl.STATIC_DRAW);

  // Create vertex shader
  const vertexShader = gl.createShader(gl.VERTEX_SHADER)!;
  gl.shaderSource(vertexShader, vertexShaderSource);
  gl.compileShader(vertexShader);

  // Check vertex shader compilation
  if (!gl.getShaderParameter(vertexShader, gl.COMPILE_STATUS)) {
    console.error(
      "Vertex shader compilation error:",
      gl.getShaderInfoLog(vertexShader)
    );
    gl.deleteShader(vertexShader);
    throw new Error("Failed to compile vertex shader");
  }

  // Create framebuffers for ping-pong
  const createFramebuffer = () => {
    const framebuffer = gl.createFramebuffer();
    const texture = gl.createTexture();

    gl.bindTexture(gl.TEXTURE_2D, texture);
    gl.texImage2D(
      gl.TEXTURE_2D,
      0,
      gl.RGBA,
      width,
      height,
      0,
      gl.RGBA,
      gl.UNSIGNED_BYTE,
      null
    );
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.LINEAR);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.LINEAR);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);

    gl.bindFramebuffer(gl.FRAMEBUFFER, framebuffer);
    gl.framebufferTexture2D(
      gl.FRAMEBUFFER,
      gl.COLOR_ATTACHMENT0,
      gl.TEXTURE_2D,
      texture,
      0
    );

    return { framebuffer, texture };
  };

  const framebuffers = [createFramebuffer(), createFramebuffer()];
  let currentFramebufferIndex = 0;

  function drawToScreen() {
    gl.bindFramebuffer(gl.FRAMEBUFFER, null);
    gl.drawArrays(gl.TRIANGLE_STRIP, 0, 4);
  }

  return {
    gl,
    width,
    height,
    framebuffers,
    vertexShader,
    drawToScreen,

    rotateFramebuffers() {
      const aBuffer = framebuffers[currentFramebufferIndex];
      currentFramebufferIndex = (currentFramebufferIndex + 1) % 2;
      const bBuffer = framebuffers[currentFramebufferIndex];
      return {
        aBuffer,
        bBuffer,
      };
    },
  };
}

export type Gl2dContext = ReturnType<typeof Gl2dContext>;
