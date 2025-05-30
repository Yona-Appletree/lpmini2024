import { UnionDef } from "@/frontend/util/zod/union-def.ts";
import { GlBlurModule } from "@/core/program/modules/gl-blur-module.tsx";
import { GlCheckerboardModule } from "@/core/program/modules/gl-checkerboard-module.tsx";
import { GlPolarScrollNode } from "@/core/program/modules/gl-polar-scroll-module.tsx";
import { GlRotateNode } from "@/core/program/modules/gl-rotate-module.tsx";
import { OscillatorModule } from "@/core/program/modules/oscillator-module.tsx";
import { z } from "zod";
import { GlPerlinModule } from "./gl-perlin-module.tsx";
import { GlMonoToHueModule } from "./gl-mono-to-hue-module.tsx";
import { GlFluidModule } from "@/core/program/modules/gl-fluid-module.tsx";

// -----------------------------------------------------------------------------
// Module definitions
//

const moduleDefs = [
  GlCheckerboardModule,
  OscillatorModule,
  GlRotateNode,
  GlPolarScrollNode,
  GlBlurModule,
  GlPerlinModule,
  GlMonoToHueModule,
  GlFluidModule,
] as const;

// -----------------------------------------------------------------------------
// Modules by type id
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

export const _index = UnionDef(
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
