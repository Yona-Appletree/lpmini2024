import { CheckerboardStage } from "./checkerboard/checkerboard.stage.tsx";
import { defaultCanvasSize, type EffectNode } from "./effect-node.ts";
import { Size2d } from "../util/size2d.ts";

export function Pipeline() {
  const config: PipelineConfig = {
    size: defaultCanvasSize,
  };

  return {
    config,
    stages: [CheckerboardStage()],
  };
}

export interface Pipeline {
  config: PipelineConfig;
  stages: EffectNode[];
}

export interface PipelineConfig {
  size: Size2d;
}
