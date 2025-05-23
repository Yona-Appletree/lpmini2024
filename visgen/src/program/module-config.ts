import { UnionDef } from "../util/zod/union-def";
import { GlBlurModule } from "@/program/modules/gl-blur-module.tsx";
import { GlCheckerboardModule } from "@/program/modules/gl-checkerboard-module.tsx";
import { GlHslShiftModule } from "@/program/modules/gl-hsl-shift-module.tsx";
import { GlPolarScrollNode } from "@/program/modules/gl-polar-scroll-module.tsx";
import { GlRotateNode } from "@/program/modules/gl-rotate-module.tsx";
import { OscillatorModule } from "@/program/modules/oscillator-module.tsx";
import { z } from "zod";

// -----------------------------------------------------------------------------
// nodeDefs
//

const moduleDefs = [
  GlCheckerboardModule,
  OscillatorModule,
  GlRotateNode,
  GlPolarScrollNode,
  GlHslShiftModule,
  GlBlurModule,
] as const;

// -----------------------------------------------------------------------------
// nodeDefByType
//

export const moduleDefByType = Object.fromEntries(
  moduleDefs.map((nodeDef) => [nodeDef.type, nodeDef]),
) as {
  [I in keyof typeof moduleDefs as (typeof moduleDefs)[I] extends {
    type: string;
  }
    ? (typeof moduleDefs)[I]["type"]
    : never]: (typeof moduleDefs)[I];
};

export type ModuleDef = (typeof moduleDefByType)[keyof typeof moduleDefByType];

// -----------------------------------------------------------------------------
// NodeConfig
//

export const ModuleConfig = UnionDef(
  "type",
  moduleDefs.map(
    (nodeDef) => nodeDef.Config.schema,
  ) as unknown as MapModuleDefsToSchemas<typeof moduleDefs>,
);
export type ModuleConfig = z.output<
  (typeof moduleDefByType)[keyof typeof moduleDefByType]["Config"]["schema"]
>;

type MapModuleDefsToSchemas<T extends readonly unknown[]> = T extends readonly [
  infer First,
  ...infer Rest,
]
  ? readonly [
      First extends { Config: { schema: unknown } }
        ? First["Config"]["schema"]
        : never,
      ...MapModuleDefsToSchemas<Rest>,
    ]
  : readonly [];
