import { Size2d } from "../util/size2d.ts";
import React from "react";
import type { Gl2d } from "../gl2d/gl2d.ts";
import type { BaseEffectParam } from "../effect-param/base-effect-param.ts";

export interface EffectNode {
  config: Record<string, BaseEffectParam>;
  apply: (gl2d: Gl2d) => Promise<void> | void;
  component: () => React.ReactElement;
}

export interface PipelineStageConfig {
  name: string;
}

export interface PipelineConfig {
  size: Size2d;
}

export const defaultCanvasSize = Size2d(256, 256);
