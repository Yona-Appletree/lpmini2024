import { useEffect, useRef } from "react";
import { ProgramRuntime } from "@/program/program-runtime.ts";
import { demoConfig } from "./demo-graph.ts";
import { ModuleComponent } from "../module-component.tsx";

export function Demo() {
  const runtimeRef = useRef<ProgramRuntime | null>(null);
  const animationRef = useRef<number>(0);

  useEffect(() => {
    if (!runtimeRef.current) {
      runtimeRef.current = ProgramRuntime(demoConfig);
    }

    const animate = () => {
      runtimeRef.current!.tick();
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
  }, []);

  const runtime = runtimeRef.current;

  return (
    runtime && (
      <div>
        {Array.from(runtimeRef.current?.nodeMap?.entries() ?? []).map(
          ([id, node]) => (
            <ModuleComponent key={id} context={runtime} node={node} />
          ),
        )}
      </div>
    )
  );
}
