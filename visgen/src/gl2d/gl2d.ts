import { createCanvas } from "../util/create-canvas.ts";
import { Gl2dContext } from "./gl2d-context.ts";
import { curry } from "../util/curry.ts";
import { Gl2dFragmentShader } from "./gl2d-fragment-shader.ts";
import { gl2dClear } from "./gl2d-clear.ts";
import { gl2dDrawImage } from "./gl2d-draw-image.ts";
import { Gl2dBlur } from "./operations/gl2d-blur.ts";
import { Gl2dCheckerboard } from "./operations/gl2d-checkerboard.ts";
import { Gl2dHslShift } from "./operations/gl2d-hsl-shift.ts";
import { Gl2dPolarScroll } from "./operations/gl2d-polar-scroll.ts";
import { Gl2dRotate } from "./operations/gl2d-rotate.ts";

export function Gl2d(canvas = createCanvas()) {
  const context = Gl2dContext(canvas);

  return {
    context,
    fragmentShader: curry(Gl2dFragmentShader, context),
    clear: curry(gl2dClear, context),
    drawImage: curry(gl2dDrawImage, context),

    ops: {
      blur: Gl2dBlur(context),
      checkerboard: Gl2dCheckerboard(context),
      hslShift: Gl2dHslShift(context),
      polarScroll: Gl2dPolarScroll(context),
      rotate: Gl2dRotate(context),
    },
  };
}

export type Gl2d = ReturnType<typeof Gl2d>;
