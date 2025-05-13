import { defaultCanvasSize } from "../pipeline/pipeline-stage";

export function createCanvas(size = defaultCanvasSize): HTMLCanvasElement {
  const canvas = document.createElement("canvas");
  canvas.width = size.width;
  canvas.height = size.height;

  return canvas;
}
