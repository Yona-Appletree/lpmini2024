import type { RuntimeContext, RuntimeNode } from "@/program/program-runtime.ts";

export function GraphNodeComponent({
  context,
  node,
}: {
  context: RuntimeContext;
  node: RuntimeNode;
}) {
  const PreviewComponent = node.instance.component;

  const InputComponent = node.nodeDef.metadata.input.component;

  return (
    <div className="border-1 p-1">
      <div>
        {node.id}: {node.nodeDef.metadata.label}
      </div>
      <div>
        <div>input</div>

        <InputComponent
          context={context}
          meta={node.nodeDef.metadata.input.info.meta}
          currentValue={node.input}
          onChange={(newValue) => {
            node.input = newValue;
          }}
        />
      </div>
      <div>
        <PreviewComponent input={node.input} output={node.output} />
      </div>
    </div>
  );
}
