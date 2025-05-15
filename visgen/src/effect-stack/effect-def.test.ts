import { EffectDef } from "./effect-def.ts";
import { Vec4Param } from "../effect-param/params/vec4-param.ts";
import { IntParam } from "../effect-param/params/int-param.ts";
import { glsl } from "../util/glsl.ts";
import { expectTypeOf, test } from "vitest";
import { z } from "zod";
import { FloatParam } from "../effect-param/params/float-param.ts";
import { Vec2Param } from "../effect-param/params/vec2-param.ts";

test("config schema type", () => {
  // eslint-disable-next-line unused-imports/no-unused-vars
  const TestEffect = EffectDef(
    "test",
    {
      params: {
        int: IntParam({ default: 8 }),
        float: FloatParam({ default: 0.5 }),
        vec2: Vec2Param({ default: [0, 0] }),
        vec3: Vec2Param({ default: [0, 0] }),
        vec4: Vec4Param({ default: [1, 1, 1, 1] }),
      },
    },
    glsl`test`,
  );

  expectTypeOf<z.output<typeof TestEffect.Config.schema>>().toEqualTypeOf<{
    type: "test";
    args: {
      int: number;
      float: number;
      vec2: [number, number];
      vec3: [number, number];
      vec4: [number, number, number, number];
    };
  }>();
});
