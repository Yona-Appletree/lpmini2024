import type { TypeValue } from "../data/type-spec.ts";
import type { JSX } from "react";
import type { ModuleMetadata } from "./define-module.ts";

export interface NodeInstance<TMeta extends ModuleMetadata = ModuleMetadata> {
  update: (args: {
    input: TypeValue<TMeta["input"]>;
  }) => TypeValue<TMeta["output"]>;
  component: (props: {
    input: TypeValue<TMeta["input"]>;
    output: TypeValue<TMeta["output"]>;
  }) => JSX.Element;
  [Symbol.dispose]?: () => void;
}
