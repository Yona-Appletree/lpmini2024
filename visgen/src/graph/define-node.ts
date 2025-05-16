import { TypedObjectDef } from "../util/zod/typed-object-def.ts";
import { configSchemaFor } from "../config/config-schema-for.ts";
import type { NodeInstance } from "./node-def.ts";
import type { TypeSpec } from "../type/type-spec.ts";

export function defineNode<TId extends string, TMeta extends NodeMetadata>(
  type: TId,
  metadata: TMeta,
  nodeFn: () => NodeInstance<TMeta>,
) {
  const Config = TypedObjectDef(type, {
    input: configSchemaFor(metadata.input),
  });

  return Object.assign(nodeFn, { type, Config, metadata } as const);
}

export interface NodeMetadata<
  TInput extends TypeSpec = TypeSpec,
  TOutput extends TypeSpec = TypeSpec,
> {
  label: string;
  input: TInput;
  output: TOutput;
}
