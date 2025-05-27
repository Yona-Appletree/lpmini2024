import type { Vec2 } from "@/core/data/types/vec2-def.tsx";
import { createCanvas2d } from "@/frontend/util/create-canvas-2d.ts";
import type {
  ChangingImage,
  ImageChangedEvent,
} from "@/frontend/util/changing-image.ts";

export function TimeSeriesCanvas({
  pixelTime = 100,
  size = [400, 64],
  startTime = Date.now(),
  lineWidth = 2,
}: {
  pixelTime?: number;
  size?: Vec2;
  startTime?: number;
  lineWidth?: number;
} = {}) {
  const [width, height] = size;

  const { canvas, context } = createCanvas2d([width, height]);
  const listeners: ImageChangedEvent[] = [];

  // fill black
  context.fillStyle = "black";
  context.fillRect(0, 0, width, height);

  let lastValue: number | null = null;
  let lastTime = startTime;

  return {
    currentImage: () => {
      return {
        image: canvas,
        width,
        height,
      };
    },
    addChangeListener(listener: ImageChangedEvent) {
      listeners.push(listener);
      return () => {
        const index = listeners.indexOf(listener);
        if (index !== -1) {
          listeners.splice(index, 1);
        }
      };
    },
    add(value: number, time = Date.now()) {
      const deltaTime = time - lastTime;
      const deltaPixels = Math.floor(deltaTime / pixelTime);

      if (deltaPixels < 1) {
        return;
      }

      const x = width - lineWidth / 2;
      const y = height * (1 - value);

      // shift image to the left by deltaPixels
      context.drawImage(canvas, -deltaPixels, 0);

      // fill with black
      context.fillStyle = "black";
      context.fillRect(width - deltaPixels, 0, deltaPixels, height);

      // draw grid line if gridTime has passed
      // if (time - lastGridTime > gridTime) {
      //   context.fillStyle = "gray";
      //   context.fillRect(width - lineWidth, 0, lineWidth, height);

      //   lastGridTime = time;
      // }

      // draw white line connecting lastValue to value
      if (lastValue !== null) {
        const lastX = x - deltaPixels;
        const lastY = height - lastValue * height;

        context.strokeStyle = "white";
        context.lineWidth = 2;
        context.beginPath();
        context.moveTo(x, y);
        context.lineTo(lastX, lastY);
        context.stroke();
        context.closePath();
      }

      listeners.forEach((listener) => listener(canvas));

      lastValue = value;
      lastTime = time;
    },
  } satisfies TimeSeriesCanvas;
}

export interface TimeSeriesCanvas extends ChangingImage {
  add(value: number, time?: number): void;
}
