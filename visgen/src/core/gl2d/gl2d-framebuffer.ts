export type Gl2dFramebufferOptions = {
  format?: "rgba" | "float32";
  filter?: "linear" | "nearest";
};

type Gl2dFramebufferParams = {
  gl: WebGL2RenderingContext;
  width: number;
  height: number;
  options?: Gl2dFramebufferOptions;
};

export function Gl2dFramebuffer({
  gl,
  width,
  height,
  options = {},
}: Gl2dFramebufferParams) {
  const format = options.format ?? "rgba";
  const filter = options.filter ?? "linear";

  // Create framebuffer and texture
  const framebuffer = gl.createFramebuffer();
  const texture = gl.createTexture();

  gl.bindTexture(gl.TEXTURE_2D, texture);

  // Configure texture based on format
  if (format === "float32") {
    gl.texImage2D(
      gl.TEXTURE_2D,
      0,
      gl.RGBA32F,
      width,
      height,
      0,
      gl.RGBA,
      gl.FLOAT,
      null
    );
  } else {
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
  }

  // Configure texture filtering
  const filterMode = filter === "linear" ? gl.LINEAR : gl.NEAREST;
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, filterMode);
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, filterMode);
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);

  // Attach texture to framebuffer
  gl.bindFramebuffer(gl.FRAMEBUFFER, framebuffer);
  gl.framebufferTexture2D(
    gl.FRAMEBUFFER,
    gl.COLOR_ATTACHMENT0,
    gl.TEXTURE_2D,
    texture,
    0
  );

  // Check framebuffer status
  const status = gl.checkFramebufferStatus(gl.FRAMEBUFFER);
  if (status !== gl.FRAMEBUFFER_COMPLETE) {
    throw new Error(`Framebuffer is not complete: ${status}`);
  }

  return {
    framebuffer,
    texture,
    width,
    height,
    format,

    bind() {
      gl.bindFramebuffer(gl.FRAMEBUFFER, framebuffer);
    },

    unbind() {
      gl.bindFramebuffer(gl.FRAMEBUFFER, null);
    },

    bindTexture(unit = 0) {
      gl.activeTexture(gl.TEXTURE0 + unit);
      gl.bindTexture(gl.TEXTURE_2D, texture);
    },

    [Symbol.dispose]() {
      gl.deleteFramebuffer(framebuffer);
      gl.deleteTexture(texture);
    },
  };
}

export type Gl2dFramebuffer = ReturnType<typeof Gl2dFramebuffer>;
