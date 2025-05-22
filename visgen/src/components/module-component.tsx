import type {
  ModuleInstance,
  RuntimeContext,
} from "@/program/program-runtime.ts";
import { ConfigNodeComponent } from "@/config/components/config-node.component.tsx";

export function ModuleComponent({
  context,
  node,
}: {
  context: RuntimeContext;
  node: ModuleInstance;
}) {
  const PreviewComponent = node.instance.component;

  return (
    <div className="border-1 inline-flex flex-col gap-2 rounded">
      <div className="flex gap-2 items-center p-1 bg-muted">
        <span className="font-mono font-bold">{node.id}</span>
        <em className="text-sm text-muted-foreground">
          ({node.nodeDef.metadata.label})
        </em>
      </div>
      <div className="flex justify-center">
        <PreviewComponent
          context={context}
          input={node.input}
          output={node.output}
        />
      </div>
      <div>
        <ConfigNodeComponent
          value={node.nodeConfig.input}
          programConfig={context.programConfig}
          typeSpec={node.nodeDef.metadata.input}
        />
      </div>
    </div>
  );
}
