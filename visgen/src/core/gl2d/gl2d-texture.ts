import type { Gl2dContext } from "@/core/gl2d/gl2d-context.ts";

export type Gl2dTextureOptions = {
  format?: "uint8" | "float32";
  filter?: "linear" | "nearest";
  wrap?: "clamp" | "repeat";
};

export type Gl2dTextureParams = {
  width?: number;
  height?: number;
  data?: Uint8Array | Float32Array | null;
  options?: Gl2dTextureOptions;
};

export function Gl2dTexture(
  context: Gl2dContext,
  {
    width = context.gl.canvas.width,
    height = context.gl.canvas.height,
    data = null,
    options = {},
  }: Gl2dTextureParams = {},
) {
  const { gl } = context;
  const format = options.format ?? "uint8";
  const filter = options.filter ?? "linear";
  const wrap = options.wrap ?? "clamp";

  const texture = gl.createTexture();
  if (!texture) {
    throw new Error("Failed to create texture");
  }

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
      data,
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
      data,
    );
  }

  // Configure texture filtering
  const filterMode = filter === "linear" ? gl.LINEAR : gl.NEAREST;
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, filterMode);
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, filterMode);

  // Configure texture wrapping
  const wrapMode = wrap === "clamp" ? gl.CLAMP_TO_EDGE : gl.REPEAT;
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, wrapMode);
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, wrapMode);

  function bind(unit = 0) {
    gl.activeTexture(gl.TEXTURE0 + unit);
    gl.bindTexture(gl.TEXTURE_2D, texture);
  }

  return {
    $type: "Gl2dTexture",
    texture,
    width,
    height,
    format,

    bind,

    updateImage(image: TexImageSource) {
      gl.texImage2D(
        gl.TEXTURE_2D,
        0,
        gl.RGBA,
        gl.RGBA,
        gl.UNSIGNED_BYTE,
        image as TexImageSource,
      );
    },

    updateData(newData: Uint8Array | Float32Array) {
      gl.bindTexture(gl.TEXTURE_2D, texture);
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
          newData,
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
          newData,
        );
      }
    },

    drawToScreen() {
      bind();
      gl.useProgram(context.copyProgram);
      gl.bindFramebuffer(gl.FRAMEBUFFER, null);
      gl.clearColor(0, 0, 0, 1);
      gl.clear(gl.COLOR_BUFFER_BIT);
      gl.drawArrays(gl.TRIANGLE_STRIP, 0, 4);
    },

    [Symbol.dispose]() {
      gl.deleteTexture(texture);
    },
  };
}

export type Gl2dTexture = ReturnType<typeof Gl2dTexture>;
