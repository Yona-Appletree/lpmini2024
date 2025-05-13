import { useEffect, useRef, useState } from "react";
import { Gl2d } from "../../gl2d/gl2d";

export function Demo() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const gl2dRef = useRef<Gl2d | null>(null);
  const [speed, setSpeed] = useState(2.5); // Speed in seconds for one complete cycle
  const animationRef = useRef<number>(0);

  useEffect(() => {
    if (!canvasRef.current) return;

    // Create Gl2d instance once and store it in ref
    if (!gl2dRef.current) {
      const gl2d = Gl2d(canvasRef.current);
      gl2dRef.current = Gl2d(canvasRef.current);
    }

    const gl2d = gl2dRef.current;

    const startTime = performance.now();

    const animate = (currentTime: number) => {
      if (!gl2dRef.current) return;

      // Clear the canvas
      gl2d.clear();

      // Draw checkerboard
      gl2d.ops.checkerboard.draw([1, 0.5, 0, 1], [0, 0, 0.5, 1]);

      // Rotate
      gl2d.ops.rotate.draw(0, fracTimeCosine(6000, { min: -1, max: 1 }));

      // Apply polar scroll with animated offset
      gl2d.ops.polarScroll.draw(fracTimeSawtooth(2500));

      // Blur
      gl2d.ops.blur.draw(25, 0.25);

      // Apply HSL shift
      gl2d.ops.hslShift.draw(fracTimeCosine(3500), 0, 0);

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

function fracTimeSawtooth(
  periodMs: number,
  { nowMs = Date.now(), min = 0, max = 1 } = {}
) {
  const elapsedMs = nowMs;
  const frac = elapsedMs % periodMs;
  return min + (max - min) * (frac / periodMs);
}

function fracTimeCosine(
  periodMs: number,
  { nowMs = Date.now(), min = 0, max = 1 } = {}
) {
  const elapsedMs = nowMs;
  const frac = elapsedMs % periodMs;
  return (
    min + (max - min) * (0.5 * (1 - Math.cos((frac / periodMs) * 2 * Math.PI)))
  );
}
