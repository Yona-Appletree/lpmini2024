import { useEffect, useState } from "react";
import { ProgramRuntime } from "@/program/program-runtime.ts";
import { demoConfig } from "./demo-graph.ts";
import { ModuleComponent } from "../module-component.tsx";

export function Demo() {
  const [runtime] = useState(() => ProgramRuntime(demoConfig));

  useEffect(() => {
    runtime.start();
    return () => {
      runtime.stop();
    };
  }, [runtime]);

  return (
    runtime && (
      <div className="flex flex-wrap gap-2">
        {Array.from(runtime?.nodeMap?.entries() ?? []).map(([id, node]) => (
          <ModuleComponent key={id} context={runtime} node={node} />
        ))}
      </div>
    )
  );
}
