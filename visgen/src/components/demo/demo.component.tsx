import { useEffect, useRef, useState } from "react";
import { FragmentShaderCanvas } from "../../util/fragment-shader-canvas.ts";
import { runCheckerboardShader } from "./checkerboard-shader.tsx";
import { polarScrollShader } from "./polar-scroll-shader.tsx";
import { hslShiftShader } from "./hsl-shift-shader.tsx";

export function Demo() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [speed, setSpeed] = useState(2.5); // Speed in seconds for one complete cycle
  const animationRef = useRef<number>();

  useEffect(() => {
    if (!canvasRef.current) return;

    const canvas = FragmentShaderCanvas(canvasRef.current);
    let startTime = performance.now();

    const animate = (currentTime: number) => {
      // Calculate progress from 0 to 1 based on speed
      const elapsed = (currentTime - startTime) / 1000; // Convert to seconds
      const progress = (elapsed % speed) / speed;

      // Clear the canvas
      canvas.clear();

      // Draw checkerboard
      runCheckerboardShader(canvas, [1, 0.5, 0, 1], [0, 1, 0, 1]);

      // Apply polar scroll with animated offset
      polarScrollShader(canvas, progress);

      // Apply HSL shift
      hslShiftShader(canvas, progress, 0, 0);

      canvas.drawToScreen();

      // Continue animation
      animationRef.current = requestAnimationFrame(animate);
    };

    // Start animation
    animationRef.current = requestAnimationFrame(animate);

    // Cleanup
    return () => {
      if (animationRef.current) {
        cancelAnimationFrame(animationRef.current);
      }
    };
  }, [speed]); // Re-run effect when speed changes

  return (
    <div
      style={{
        width: "100vw",
        height: "100vh",
        overflow: "hidden",
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
        justifyContent: "center",
      }}
    >
      <canvas
        ref={canvasRef}
        width={256}
        height={256}
        style={{ marginBottom: "20px" }}
      />
      <div style={{ width: "200px" }}>
        <input
          type="range"
          min="1"
          max="10"
          step="0.1"
          value={speed}
          onChange={(e) => setSpeed(parseFloat(e.target.value))}
          style={{ width: "100%" }}
        />
        <div style={{ textAlign: "center", marginTop: "5px" }}>
          Speed: {speed.toFixed(1)}s
        </div>
      </div>
    </div>
  );
}
