import { useEffect, useRef } from "react";
import { FragmentShaderCanvas } from "../../util/fragment-shader-canvas.ts";
import { blurShader } from "./blur-shader.tsx";
import { checkerboardShader } from "./checkerboard-shader.tsx";

export function Demo() {
  const canvasRef = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    if (!canvasRef.current) return;

    const canvas = FragmentShaderCanvas(canvasRef.current);

    // Clear the canvas
    canvas.clear();

    // Draw checkerboard
    canvas.runShader(checkerboardShader);

    // Apply blur
    canvas.runShader(blurShader);

    canvas.drawToScreen();
  }, []);

  return (
    <div style={{ width: "100vw", height: "100vh", overflow: "hidden" }}>
      <canvas ref={canvasRef} width={128} height={128} />
    </div>
  );
}
