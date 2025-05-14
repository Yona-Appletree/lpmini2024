import { Gl2dBlur } from "./effects/gl2d-blur.ts";
import { Gl2dCheckerboard } from "./effects/gl2d-checkerboard.ts";
import { Gl2dHslShift } from "./effects/gl2d-hsl-shift.ts";
import { Gl2dPolarScroll } from "./effects/gl2d-polar-scroll.ts";
import { Gl2dRotate } from "./effects/gl2d-rotate.ts";
import type { EffectArguments } from "../effect-param/effect-arguments.ts";
import { z } from "zod";

export const Gl2dEffects = {
  [Gl2dBlur.type]: Gl2dBlur,
  [Gl2dCheckerboard.type]: Gl2dCheckerboard,
  [Gl2dHslShift.type]: Gl2dHslShift,
  [Gl2dPolarScroll.type]: Gl2dPolarScroll,
  [Gl2dRotate.type]: Gl2dRotate,
};

export type Gl2dEffect = (typeof Gl2dEffects)[keyof typeof Gl2dEffects];
export type Gl2dEffectType = keyof typeof Gl2dEffects;

export type EffectArgsFor<TType extends Gl2dEffectType> = EffectArguments<
  (typeof Gl2dEffects)[TType]["metadata"]["params"]
>;
