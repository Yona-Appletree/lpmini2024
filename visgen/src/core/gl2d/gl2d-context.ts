import { createCanvas } from "@/frontend/util/create-canvas.ts";
import { glsl } from "@/frontend/util/glsl.ts";

interface Gl2dContextReturn {
  gl: WebGL2RenderingContext;
  width: number;
  height: number;
  copyProgram: WebGLProgram;
  vertexShader: WebGLShader;
  copyFragmentShader: WebGLShader;
  [Symbol.dispose]: () => void;
}

export function Gl2dContext(canvas = createCanvas()): Gl2dContextReturn {
  const gl = canvas.getContext("webgl2")!;
  const width = canvas.width;
  const height = canvas.height;

  // Enable floating point texture support
  const ext = gl.getExtension("EXT_color_buffer_float");
  if (!ext) {
    throw new Error("Floating point textures not supported on this device");
  }

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

  // Create a simple program to draw the image
  const copyFragmentShader = gl.createShader(gl.FRAGMENT_SHADER)!;
  gl.shaderSource(
    copyFragmentShader,
    glsl`
      #version 300 es
      precision highp float;
      in vec2 vUv;
      uniform sampler2D uTexture;
      out vec4 fragColor;
      void main() {
        fragColor = texture(uTexture, vUv);
      }
    `
  );
  gl.compileShader(copyFragmentShader);

  // Check fragment shader compilation
  if (!gl.getShaderParameter(copyFragmentShader, gl.COMPILE_STATUS)) {
    console.error(
      "Fragment shader compilation error:",
      gl.getShaderInfoLog(copyFragmentShader)
    );
    gl.deleteShader(copyFragmentShader);
    throw new Error("Failed to compile fragment shader");
  }

  const copyProgram = gl.createProgram()!;
  gl.attachShader(copyProgram, vertexShader);
  gl.attachShader(copyProgram, copyFragmentShader);
  gl.linkProgram(copyProgram);

  // Check program linking
  if (!gl.getProgramParameter(copyProgram, gl.LINK_STATUS)) {
    console.error("Program linking error:", gl.getProgramInfoLog(copyProgram));
    gl.deleteProgram(copyProgram);
    gl.deleteShader(copyFragmentShader);
    throw new Error("Failed to link program");
  }

  return {
    gl,
    width,
    height,
    copyProgram,
    vertexShader,
    copyFragmentShader,

    [Symbol.dispose]() {
      gl.deleteProgram(copyProgram);
      gl.deleteShader(vertexShader);
      gl.deleteShader(copyFragmentShader);
      gl.deleteBuffer(positionBuffer);
    },
  };
}

export type Gl2dContext = Gl2dContextReturn;
