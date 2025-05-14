import { createCanvas } from "../util/create-canvas.ts";
import { Gl2dContext } from "./gl2d-context.ts";
import { curry } from "../util/curry.ts";
import { Gl2dFragmentShader } from "./gl2d-fragment-shader.ts";
import { gl2dClear } from "./gl2d-clear.ts";
import { gl2dDrawImage } from "./gl2d-draw-image.ts";

export function Gl2d(canvas = createCanvas()) {
  const context = Gl2dContext(canvas);

  return {
    context,
    fragmentShader: curry(Gl2dFragmentShader, context),
    clear: curry(gl2dClear, context),
    drawImage: curry(gl2dDrawImage, context),
  };
}

export type Gl2d = ReturnType<typeof Gl2d>;
