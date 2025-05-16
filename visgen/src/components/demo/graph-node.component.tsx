import type { NodeDef } from "../../graph/node-config";
import type { NodeInstance } from "../../graph/node-def";

interface Node {
  nodeDef: NodeDef;
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  instance: NodeInstance<any>;
  output: unknown;
  input: unknown;
}

interface GraphNodeProps {
  id: string;
  node: Node;
}

export function GraphNodeComponent({ id, node }: GraphNodeProps) {
  const PreviewComponent = node.instance.component;

  return (
    <div className="border-1 p-1">
      <div>
        {id}: {node.nodeDef.metadata.label}
      </div>
      <div>
        <div>input</div>
      </div>
      <div>
        <PreviewComponent input={node.input} output={node.output} />
      </div>
    </div>
  );
}
