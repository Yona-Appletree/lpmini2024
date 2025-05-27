import type { CanvasHTMLAttributes } from "react";
import { useEffect, useRef } from "react";
import type { ChangingImage } from "@/frontend/util/changing-image.ts";

interface CanvasImageProps extends CanvasHTMLAttributes<HTMLCanvasElement> {
  image: ChangingImage;
}

export function CanvasImage({ image, ...props }: CanvasImageProps) {
  const canvasRef = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const context = canvas.getContext("2d");
    if (!context) return;

    // Initial draw
    const drawImage = () => {
      const img = image.currentImage();

      if (img) {
        // Clear the canvas
        context.clearRect(0, 0, canvas.width, canvas.height);

        // Draw the image, scaled to fit
        const scale = Math.min(
          canvas.width / img.width,
          canvas.height / img.height,
        );
        const x = (canvas.width - img.width * scale) / 2;
        const y = (canvas.height - img.height * scale) / 2;
        context.drawImage(
          img.image,
          x,
          y,
          img.width * scale,
          img.height * scale,
        );
      }
    };

    // Draw initially
    drawImage();

    // Subscribe to changes
    const unsubscribe = image.addChangeListener(() => {
      drawImage();
    });

    // Cleanup subscription on unmount
    return () => {
      unsubscribe();
    };
  }, [image]);

  return <canvas ref={canvasRef} {...props} />;
}
