import { useEffect, useRef, useState } from "react";
import { GraphRuntime } from "../../graph/graph-runtime.ts";
import { demoConfig } from "./demo-graph.ts";
import { GraphNodeComponent } from "../graph-node.component.tsx";

export function Demo() {
  const graphRef = useRef<GraphRuntime | null>(null);
  const animationRef = useRef<number>(0);
  const [currentTime, setCurrentTime] = useState(0);

  useEffect(() => {
    if (!graphRef.current) {
      graphRef.current = GraphRuntime(demoConfig);
    }

    const animate = () => {
      graphRef.current!.tick();
      setCurrentTime(performance.now());
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

  return (
    <div>
      {Array.from(graphRef.current?.nodeMap?.entries() ?? []).map(
        ([id, node]) => (
          <GraphNodeComponent key={id} id={id} node={node} />
        ),
      )}
    </div>
  );
}
