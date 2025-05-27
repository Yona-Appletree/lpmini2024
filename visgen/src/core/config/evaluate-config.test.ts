import { RecordDef } from "@/core/data/types/record-def.tsx";
import type { ConfigEvalContext } from "./config-eval-context.ts";
import { FloatDef } from "@/core/data/types/float-def.tsx";
import { expect, test } from "vitest";
import { evaluateConfig } from "./evaluate-config.ts";
import { Vec3Def } from "@/core/data/types/vec3-def.tsx";
import type { ConfigValueFor } from "./config-schema-for.ts";

test("basic eval", () => {
  const typeSpec = RecordDef({
    angle: FloatDef({ default: 0 }),
    swirl: FloatDef({ default: 2.0 }),
    vec: Vec3Def({ default: [0, 0, 0] }),
  });

  const config: ConfigValueFor<typeof typeSpec> = {
    value: {
      angle: { value: 10 },
      vec: {
        value: [{ value: 1 }, { value: 2 }, { value: 3 }],
      },
      swirl: {
        $moduleOutput: {
          moduleId: "lfo",
        },
        activeExpr: "$moduleOutput",
      },
    },
  };

  const context: ConfigEvalContext = {
    moduleMap: new Map([["lfo", { output: 0.5 }]]),
  };

  const result = evaluateConfig({
    spec: typeSpec,
    configNode: config,
    context,
  });

  expect(result).toEqual({
    angle: 10,
    swirl: 0.5,
    vec: [1, 2, 3],
  });
});
