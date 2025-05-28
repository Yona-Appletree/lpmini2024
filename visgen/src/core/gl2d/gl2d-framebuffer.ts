import { Gl2dTexture, type Gl2dTextureParams } from "./gl2d-texture";
import type { Gl2dContext } from "@/core/gl2d/gl2d-context.ts";

export function Gl2dFramebuffer(
  context: Gl2dContext,
  options: Gl2dTextureParams = {},
) {
  const { gl } = context;

  // Create texture
  const texture = Gl2dTexture(context, options);

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

  function bind() {
    gl.bindFramebuffer(gl.FRAMEBUFFER, framebuffer);
  }

  function unbind() {
    gl.bindFramebuffer(gl.FRAMEBUFFER, null);
  }

  function clear() {
    bind();
    gl.clearColor(0, 0, 0, 1);
    gl.clear(gl.COLOR_BUFFER_BIT);
  }

  return {
    framebuffer,
    texture,

    bind,
    unbind,
    clear,

    [Symbol.dispose]() {
      gl.deleteFramebuffer(framebuffer);
      texture[Symbol.dispose]();
    },
  };
}

export type Gl2dFramebuffer = ReturnType<typeof Gl2dFramebuffer>;
