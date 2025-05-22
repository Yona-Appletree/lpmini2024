import type { RuntimeContext, RuntimeNode } from "@/program/program-runtime.ts";

export function ModuleComponent({
  context,
  node,
}: {
  context: RuntimeContext;
  node: RuntimeNode;
}) {
  const PreviewComponent = node.instance.component;

  const InputComponent = node.nodeDef.metadata.input.component;

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
        <InputComponent
          context={context}
          meta={node.nodeDef.metadata.input.info.meta}
          currentValue={node.input}
          onChange={(newValue) => {
            node.input = newValue;
          }}
        />
      </div>
    </div>
  );
}
