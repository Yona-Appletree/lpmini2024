import { useEffect, useRef, useState } from "react";
import { Gl2d } from "../../gl2d/gl2d";
import { Gl2dHslShift } from "./gl2d-hsl-shift.ts";
import { Gl2dPolarScroll } from "./gl2d-polar-scroll.ts";
import { Gl2dCheckerboard } from "./gl2d-checkerboard.ts";

export function Demo() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const gl2dRef = useRef<{
    gl2d: Gl2d;
    checkerboard: Gl2dCheckerboard;
    hslShift: Gl2dHslShift;
    polarScroll: Gl2dPolarScroll;
  } | null>(null);
  const [speed, setSpeed] = useState(2.5); // Speed in seconds for one complete cycle
  const animationRef = useRef<number>(0);

  useEffect(() => {
    if (!canvasRef.current) return;

    // Create Gl2d instance once and store it in ref
    if (!gl2dRef.current) {
      const gl2d = Gl2d(canvasRef.current);
      gl2dRef.current = {
        gl2d,
        checkerboard: Gl2dCheckerboard(gl2d.context),
        hslShift: Gl2dHslShift(gl2d.context),
        polarScroll: Gl2dPolarScroll(gl2d.context),
      };
    }

    const { gl2d, checkerboard, hslShift, polarScroll } = gl2dRef.current;

    const startTime = performance.now();

    const animate = (currentTime: number) => {
      if (!gl2dRef.current) return;

      // Calculate progress from 0 to 1 based on speed
      const elapsed = (currentTime - startTime) / 1000; // Convert to seconds
      const progress = (elapsed % speed) / speed;

      // Clear the canvas
      gl2d.clear();

      // Draw checkerboard
      checkerboard.draw([1, 0.5, 0, 1], [0, 1, 0, 1]);

      // Apply polar scroll with animated offset
      polarScroll.draw(progress);

      // Apply HSL shift
      hslShift.draw(progress, 0, 0);

      gl2d.context.drawToScreen();

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
