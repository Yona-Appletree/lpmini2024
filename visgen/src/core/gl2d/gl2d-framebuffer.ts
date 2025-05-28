import { Gl2dTexture, type Gl2dTextureOptions } from "./gl2d-texture";

export type Gl2dFramebufferOptions = Gl2dTextureOptions;

type Gl2dFramebufferParams = {
  gl: WebGL2RenderingContext;
  texture: Gl2dTexture;
  options?: Gl2dFramebufferOptions;
};

export function Gl2dFramebuffer({ gl, texture }: Gl2dFramebufferParams) {
  // Create framebuffer
  const framebuffer = gl.createFramebuffer();
  if (!framebuffer) {
    throw new Error("Failed to create framebuffer");
  }

  // Attach texture to framebuffer
  gl.bindFramebuffer(gl.FRAMEBUFFER, framebuffer);
  gl.framebufferTexture2D(
    gl.FRAMEBUFFER,
    gl.COLOR_ATTACHMENT0,
    gl.TEXTURE_2D,
    texture.texture,
    0,
  );

  // Check framebuffer status
  const status = gl.checkFramebufferStatus(gl.FRAMEBUFFER);
  if (status !== gl.FRAMEBUFFER_COMPLETE) {
    throw new Error(`Framebuffer is not complete: ${status}`);
  }

  return {
    framebuffer,
    texture: texture.texture,

    bind() {
      gl.bindFramebuffer(gl.FRAMEBUFFER, framebuffer);
    },

    unbind() {
      gl.bindFramebuffer(gl.FRAMEBUFFER, null);
    },

    bindTexture(unit = 0) {
      texture.bind(unit);
    },

    [Symbol.dispose]() {
      gl.deleteFramebuffer(framebuffer);
      texture[Symbol.dispose]();
    },
  };
}

export type Gl2dFramebuffer = ReturnType<typeof Gl2dFramebuffer>;
