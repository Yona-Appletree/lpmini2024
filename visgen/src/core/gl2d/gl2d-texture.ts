export type Gl2dTextureOptions = {
  format?: "uint8" | "float32";
  filter?: "linear" | "nearest";
  wrap?: "clamp" | "repeat";
};

type Gl2dTextureParams = {
  gl: WebGL2RenderingContext;
  width: number;
  height: number;
  data?: Uint8Array | Float32Array | null;
  options?: Gl2dTextureOptions;
};

export function Gl2dTexture({
  gl,
  width,
  height,
  data = null,
  options = {},
}: Gl2dTextureParams) {
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

  return {
    texture,
    width,
    height,
    format,

    bind(unit = 0) {
      gl.activeTexture(gl.TEXTURE0 + unit);
      gl.bindTexture(gl.TEXTURE_2D, texture);
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

    [Symbol.dispose]() {
      gl.deleteTexture(texture);
    },
  };
}

export type Gl2dTexture = ReturnType<typeof Gl2dTexture>;
