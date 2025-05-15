import { Blur } from "./effects/blur.ts";
import { Checkerboard } from "./effects/checkerboard.ts";
import { HslShift } from "./effects/hsl-shift.ts";
import { PolarScroll } from "./effects/polar-scroll.ts";
import { Rotate } from "./effects/rotate.ts";
import type { EffectArguments } from "../effect-param/effect-arguments.ts";
import { expectTypeOf } from "vitest";
import { UnionDef } from "../util/zod/union-def.ts";

export const effectByType = {
  [Blur.type]: Blur,
  [Checkerboard.type]: Checkerboard,
  [HslShift.type]: HslShift,
  [PolarScroll.type]: PolarScroll,
  [Rotate.type]: Rotate,
};
export const EffectConfig = UnionDef("type", [
  Blur.Config.schema,
  Checkerboard.Config.schema,
  HslShift.Config.schema,
  PolarScroll.Config.schema,
  Rotate.Config.schema,
]);

Checkerboard.Config({
  args: {
    color1: [1, 0.5, 0, 1],
    color2: [0, 0, 0.5, 1],
    rows: 10,
    columns: 10,
  },
});

// Ensure that the above effectByType and EffectConfig are in sync
// unfortunately, there is no easy way to make this dry and type-safe
expectTypeOf<
  (typeof EffectConfig.schema.options)[number]["shape"]["type"]["value"]
>().toEqualTypeOf<EffectType>();

export type Effect = (typeof effectByType)[keyof typeof effectByType];
export type EffectType = keyof typeof effectByType;

export type EffectArgsFor<TType extends EffectType> = EffectArguments<
  (typeof effectByType)[TType]["metadata"]["params"]
>;

export type EffectConfig = ReturnType<typeof EffectConfig>;
