import { defaultCanvasSize } from "../pipeline/pipeline-stage";
import { createCanvas } from "./create-canvas";

export function FragmentShaderCanvas(canvas = createCanvas()) {
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
      gl.getShaderInfoLog(vertexShader),
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
      null,
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
      0,
    );

    return { framebuffer, texture };
  };

  const framebuffers = [createFramebuffer(), createFramebuffer()];
  let currentFramebufferIndex = 0;

  function clear() {
    gl.bindFramebuffer(
      gl.FRAMEBUFFER,
      framebuffers[currentFramebufferIndex].framebuffer,
    );
    gl.clearColor(0, 0, 0, 1);
    gl.clear(gl.COLOR_BUFFER_BIT);
  }

  function drawImage(image: CanvasImageSource) {
    gl.bindFramebuffer(
      gl.FRAMEBUFFER,
      framebuffers[currentFramebufferIndex].framebuffer,
    );
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
      varying vec2 vUv;
      uniform sampler2D uTexture;
      void main() {
        gl_FragColor = texture2D(uTexture, vUv);
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

  function runShader(shaderGlsl: string) {
    // Create fragment shader
    const fragmentShader = gl.createShader(gl.FRAGMENT_SHADER)!;
    gl.shaderSource(fragmentShader, shaderGlsl);
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

    // Create program
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

    // Set up position attribute
    const positionLocation = gl.getAttribLocation(program, "aVertexPosition");
    gl.enableVertexAttribArray(positionLocation);
    gl.vertexAttribPointer(positionLocation, 2, gl.FLOAT, false, 0, 0);

    // Set resolution uniform
    const resolutionLocation = gl.getUniformLocation(program, "uResolution");
    if (resolutionLocation !== null) {
      gl.uniform2f(resolutionLocation, width, height);
    }

    // Set input texture uniform
    const inputTextureLocation = gl.getUniformLocation(
      program,
      "uInputTexture",
    );
    if (inputTextureLocation !== null) {
      gl.uniform1i(inputTextureLocation, 0);
      gl.activeTexture(gl.TEXTURE0);
      gl.bindTexture(
        gl.TEXTURE_2D,
        framebuffers[currentFramebufferIndex].texture,
      );
    }

    // Draw to the other framebuffer
    currentFramebufferIndex = (currentFramebufferIndex + 1) % 2;
    gl.bindFramebuffer(
      gl.FRAMEBUFFER,
      framebuffers[currentFramebufferIndex].framebuffer,
    );
    gl.drawArrays(gl.TRIANGLE_STRIP, 0, 4);

    // Clean up
    gl.deleteProgram(program);
    gl.deleteShader(fragmentShader);
  }

  function drawToScreen() {
    gl.bindFramebuffer(gl.FRAMEBUFFER, null);
    gl.drawArrays(gl.TRIANGLE_STRIP, 0, 4);
  }

  return {
    canvas,
    clear,
    drawImage,
    runShader,
    drawToScreen,
  };
}
