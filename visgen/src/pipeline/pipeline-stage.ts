import { Size2d } from "../util/size2d";
import React from "react";

export interface PipelineStage {
  config: PipelineStageConfig;
  buildOperations(pipeline: PipelineConfig): Operation[];
  component: () => React.ReactElement;
}

export interface PipelineStageConfig {
  name: string;
}

export interface PipelineConfig {
  size: Size2d;
}

export const defaultCanvasSize = Size2d(256, 256);
