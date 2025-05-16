import { UnionDef } from "../util/zod/union-def";
import { GlCheckerboardNode } from "./nodes/gl-checkerboard-node";
import { GlPolarScrollNode } from "./nodes/gl-polar-scroll";
import { GlRotateNode } from "./nodes/gl-rotate";
import { LowFrequencyOscillator } from "./nodes/low-frequency-oscillator-node";

export const nodeDefByType = {
  [GlCheckerboardNode.type]: GlCheckerboardNode,
  [LowFrequencyOscillator.type]: LowFrequencyOscillator,
  [GlRotateNode.type]: GlRotateNode,
  [GlPolarScrollNode.type]: GlPolarScrollNode,
} as const;

export type NodeDef = (typeof nodeDefByType)[keyof typeof nodeDefByType];

export const NodeConfig = UnionDef("type", [
  GlCheckerboardNode.Config.schema,
  LowFrequencyOscillator.Config.schema,
  GlRotateNode.Config.schema,
  GlPolarScrollNode.Config.schema,
]);
