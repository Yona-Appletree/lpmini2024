import { UnionDef } from "../util/zod/union-def";
import { GlBlurNode } from "./nodes/gl-blur-node";
import { GlCheckerboardNode } from "./nodes/gl-checkerboard-node";
import { GlHslShiftNode } from "./nodes/gl-hsl-shift-node";
import { GlPolarScrollNode } from "./nodes/gl-polar-scroll";
import { GlRotateNode } from "./nodes/gl-rotate";
import { LowFrequencyOscillator } from "./nodes/low-frequency-oscillator-node";

// -----------------------------------------------------------------------------
// nodeDefs
//

const nodeDefs = [
  GlCheckerboardNode,
  LowFrequencyOscillator,
  GlRotateNode,
  GlPolarScrollNode,
  GlHslShiftNode,
  GlBlurNode,
] as const;

// -----------------------------------------------------------------------------
// nodeDefByType
//

export const nodeDefByType = Object.fromEntries(
  nodeDefs.map((nodeDef) => [nodeDef.type, nodeDef]),
) as {
  [I in keyof typeof nodeDefs as (typeof nodeDefs)[I] extends { type: string }
    ? (typeof nodeDefs)[I]["type"]
    : never]: (typeof nodeDefs)[I];
};

export type NodeDef = (typeof nodeDefByType)[keyof typeof nodeDefByType];

// -----------------------------------------------------------------------------
// NodeConfig
//

export const NodeConfig = UnionDef(
  "type",
  nodeDefs.map(
    (nodeDef) => nodeDef.Config.schema,
  ) as unknown as MapNodeDefsToSchemas<typeof nodeDefs>,
);
export type NodeConfig = (typeof nodeDefByType)[keyof typeof nodeDefByType];

type MapNodeDefsToSchemas<T extends readonly unknown[]> = T extends readonly [
  infer First,
  ...infer Rest,
]
  ? readonly [
      First extends { Config: { schema: unknown } }
        ? First["Config"]["schema"]
        : never,
      ...MapNodeDefsToSchemas<Rest>,
    ]
  : readonly [];
