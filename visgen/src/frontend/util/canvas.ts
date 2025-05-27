import { defaultCanvasSize } from "@/frontend/util/default-canvas-size.ts";
import type { Vec2 } from "@/core/data/types/vec2-def.tsx";

export function Canvas({ size = defaultCanvasSize }) {
  const canvas = document.createElement("canvas");
  canvas.width = size[0];
  canvas.height = size[1];

  return {
    canvas,
    get context2d() {
      const ctx = canvas.getContext("2d");
      if (!ctx) throw new Error("Failed to get context");
      return ctx;
    },
    ensureSizeContext<T extends ContextType>(
      contextType: T,
      { size: [width, height] = defaultCanvasSize }: { size?: Vec2 } = {},
    ): ContextFor<T> {
      if (canvas.width !== width || canvas.height !== height) {
        canvas.width = width;
        canvas.height = height;
      }

      const ctx = canvas.getContext(contextType);
      if (!ctx) throw new Error("Failed to get context");
      return ctx as ContextFor<T>;
    },
  };
}

export type ContextType = "2d" | "webgl" | "webgl2";

export type ContextFor<T extends ContextType> = T extends "2d"
  ? CanvasRenderingContext2D
  : T extends "webgl"
    ? WebGLRenderingContext
    : T extends "webgl2"
      ? WebGL2RenderingContext
      : never;
