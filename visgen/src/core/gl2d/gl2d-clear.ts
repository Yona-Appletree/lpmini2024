import { Gl2dContext } from "./gl2d-context.ts";

export function gl2dClear(context: Gl2dContext) {
  const { gl, framebuffers } = context;
  gl.bindFramebuffer(gl.FRAMEBUFFER, framebuffers[0].framebuffer);
  gl.clearColor(0, 0, 0, 1);
  gl.clear(gl.COLOR_BUFFER_BIT);
}
