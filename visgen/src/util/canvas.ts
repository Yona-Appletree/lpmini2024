import { defaultCanvasSize } from "../graph/effect-node.ts";
import { Size2d } from "./size2d";

export function Canvas({ size = defaultCanvasSize }) {
  const canvas = document.createElement("canvas");
  canvas.width = size.width;
  canvas.height = size.height;

  return {
    canvas,
    ensureSizeContext<T extends ContextType>(
      contextType: T,
      { size = defaultCanvasSize }: { size?: Size2d } = {},
    ): ContextFor<T> {
      if (canvas.width !== size.width || canvas.height !== size.height) {
        canvas.width = size.width;
        canvas.height = size.height;
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
