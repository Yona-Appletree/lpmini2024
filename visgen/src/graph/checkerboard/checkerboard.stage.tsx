import type {
  EffectNode,
  PipelineConfig,
  PipelineStageConfig as PipelineStageConfig,
} from "../effect-node.ts";
import { Canvas } from "../../util/canvas.ts";
import { CheckerboardComponent } from "./checkerboard.component.tsx";

export function CheckerboardStage(): EffectNode {
  const canvas = Canvas();
  const config: CheckerboardConfig = {
    name: "Checkerboard",
    rows: 8,
    columns: 8,
  };

  return {
    config: config,
    component: () => <CheckerboardComponent config={config} />,
    buildOperations: (pipeline: PipelineConfig) => {
      return `
        void main() {
          vec2 uv = gl_FragCoord.xy / uResolution;
          vec2 cell = floor(uv * ${config.rows});
          float color = mod(cell.x + cell.y, 2.0);
          gl_FragColor = vec4(vec3(color), 1.0);
        }
      `;
    },
  };
}

export interface CheckerboardConfig extends PipelineStageConfig {
  rows: number;
  columns: number;
}
