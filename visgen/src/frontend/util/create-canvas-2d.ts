import type { Vec2 } from "@/core/data/types/vec2-def.tsx";

export function createCanvas2d(size: Vec2) {
  const canvas = document.createElement("canvas");
  canvas.width = size[0];
  canvas.height = size[1];
  const context = canvas.getContext("2d");
  if (!context) {
    throw new Error("Failed to get 2d context");
  }
  return { canvas, context };
}
