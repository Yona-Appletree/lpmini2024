import type { Meta, StoryObj } from "@storybook/react";
import { TimeSeriesCanvas } from "@/lib/time-series-canvas.ts";
import { useEffect } from "react";
import { CanvasImage } from "./canvas-image";

const meta = {
  component: CanvasImage,
  parameters: {
    layout: "centered",
  },
} satisfies Meta<typeof CanvasImage>;

export default meta;
type Story = StoryObj<typeof meta>;

export const TimeSeriesGraph = {
  args: {
    width: "400px",
    height: "64px",
    timeSeries: TimeSeriesCanvas(),
  },
  render: ({ timeSeries }: { timeSeries: TimeSeriesCanvas }) => {
    useEffect(() => {
      // Set up animation
      const startTime = Date.now();
      const period = 2500; // 2.5 seconds

      let running = true;

      const animate = () => {
        if (!running) return;

        const now = Date.now();
        const elapsed = now - startTime;

        // Calculate sine wave value (between 0 and 1)
        const value = (Math.sin((elapsed / period) * Math.PI * 2) + 1) / 2;

        timeSeries.add(value);

        requestAnimationFrame(animate);
      };

      animate();

      return () => {
        running = false;
      };
    }, []);

    return <CanvasImage image={timeSeries} />;
  },
};
