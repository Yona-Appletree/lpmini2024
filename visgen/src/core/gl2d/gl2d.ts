import { createCanvas } from "@/frontend/util/create-canvas.ts";
import { Gl2dContext } from "./gl2d-context.ts";
import { curry } from "@/frontend/util/curry.ts";
import { Gl2dFragmentShader } from "./gl2d-fragment-shader.ts";
import {
  Gl2dTexture,
  type Gl2dTextureParams,
} from "@/core/gl2d/gl2d-texture.ts";
import { Gl2dFramebuffer } from "@/core/gl2d/gl2d-framebuffer.ts";

export function Gl2d(canvas = createCanvas()) {
  const context = Gl2dContext(canvas);

  return {
    canvas,
    context,
    fragmentShader: curry(Gl2dFragmentShader, context),
    texture: curry(Gl2dTexture, context),
    framebuffer: (params: Gl2dTextureParams) =>
      Gl2dFramebuffer(context, params),
  };
}

export type Gl2d = ReturnType<typeof Gl2d>;
