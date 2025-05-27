export function createCanvas(size = [256, 256]): HTMLCanvasElement {
  const canvas = document.createElement("canvas");
  canvas.width = size[0];
  canvas.height = size[1];

  return canvas;
}
