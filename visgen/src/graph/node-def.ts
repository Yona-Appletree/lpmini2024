import type { TypeValue } from "../type/type-spec.ts";
import type { JSX } from "react";
import type { NodeMetadata } from "./define-node.ts";

export interface NodeInstance<TMeta extends NodeMetadata = NodeMetadata> {
  update: (args: {
    input: TypeValue<TMeta["input"]>;
  }) => TypeValue<TMeta["output"]>;
  component: (props: {
    input: TypeValue<TMeta["input"]>;
    output: TypeValue<TMeta["output"]>;
  }) => JSX.Element;
  [Symbol.dispose]?: () => void;
}
