import { TypedObjectDef } from "../util/zod/typed-object-def.ts";
import type { TypeSpec, TypeValue } from "../type/type-spec.ts";
import { configSchemaFor } from "../config/config-schema-for.ts";

export function NodeDef<TId extends string, TMeta extends NodeMetadata>(
  type: TId,
  metadata: TMeta,
  nodeFn: () => NodeInstance<TMeta>,
) {
  const Config = TypedObjectDef(type, {
    input: configSchemaFor(metadata.input),
  });

  return Object.assign(nodeFn, { type, Config, metadata } as const);
}

interface NodeMetadata<
  TInput extends TypeSpec = TypeSpec,
  TOutput extends TypeSpec = TypeSpec,
> {
  label: string;
  input: TInput;
  output: TOutput;
}

export interface NodeInstance<TMeta extends NodeMetadata = NodeMetadata> {
  update: (args: {
    input: TypeValue<TMeta["input"]>;
    deltaMs: number;
  }) => TypeValue<TMeta["output"]>;
  [Symbol.dispose]?: () => void;
}
