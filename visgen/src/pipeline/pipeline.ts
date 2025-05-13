import { CheckerboardStage } from "./checkerboard/checkerboard.stage";
import { defaultCanvasSize, type PipelineStage } from "./pipeline-stage.ts";
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
  stages: PipelineStage[];
}

export interface PipelineConfig {
  size: Size2d;
}
