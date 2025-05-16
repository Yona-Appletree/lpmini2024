import type { Gl2dContext } from "./gl2d-context.ts";

export function Gl2dFragmentShader(canvas: Gl2dContext, shaderGlsl: string) {
  const { gl, vertexShader, width, height } = canvas;

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

  function draw(uniforms: ShaderUniformsRecord) {
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

    for (const [uniformName, uniformValue] of Object.entries(uniforms)) {
      const uniformLocation = gl.getUniformLocation(program, uniformName);
      if (uniformLocation !== null) {
        switch (uniformValue.type) {
          case "int32":
            gl.uniform1i(uniformLocation, uniformValue.value);
            break;
          case "float32":
            gl.uniform1f(uniformLocation, uniformValue.value);
            break;
          case "vec2":
            gl.uniform2f(
              uniformLocation,
              uniformValue.value[0],
              uniformValue.value[1],
            );
            break;
          case "vec3":
            gl.uniform3f(
              uniformLocation,
              uniformValue.value[0],
              uniformValue.value[1],
              uniformValue.value[2],
            );
            break;
          case "vec4":
            gl.uniform4f(
              uniformLocation,
              uniformValue.value[0],
              uniformValue.value[1],
              uniformValue.value[2],
              uniformValue.value[3],
            );
            break;
          default:
            throw new Error(`Unsupported uniform type: ${uniformValue.type}`);
        }
      }
    }

    const { aBuffer, bBuffer } = canvas.rotateFramebuffers();

    // Set input texture uniform
    const inputTextureLocation = gl.getUniformLocation(
      program,
      "uInputTexture",
    );
    if (inputTextureLocation !== null) {
      gl.uniform1i(inputTextureLocation, 0);
      gl.activeTexture(gl.TEXTURE0);
      gl.bindTexture(gl.TEXTURE_2D, aBuffer.texture);
    }

    // Draw to the other framebuffer
    gl.bindFramebuffer(gl.FRAMEBUFFER, bBuffer.framebuffer);
    gl.drawArrays(gl.TRIANGLE_STRIP, 0, 4);
  }

  return {
    draw,
    [Symbol.dispose]() {
      gl.deleteProgram(program);
      gl.deleteShader(fragmentShader);
    },
  };
}

export type Gl2dFragmentShader = ReturnType<typeof Gl2dFragmentShader>;

export type ShaderUniformsRecord = Record<
  string,
  | { type: "float32"; value: number }
  | { type: "vec2"; value: [number, number] }
  | { type: "vec3"; value: [number, number, number] }
  | { type: "vec4"; value: [number, number, number, number] }
>;
