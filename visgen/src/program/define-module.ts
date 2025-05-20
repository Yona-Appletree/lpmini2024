import { TypedObjectDef } from "../util/zod/typed-object-def.ts";
import { configSchemaFor } from "../config/config-schema-for.ts";
import type { NodeInstance } from "./module-def.ts";
import type { TypeSpec } from "../data/type-spec.ts";

export function defineModule<TId extends string, TMeta extends ModuleMetadata>(
  type: TId,
  metadata: TMeta,
  nodeFn: () => NodeInstance<TMeta>,
) {
  const Config = TypedObjectDef(type, {
    input: configSchemaFor(metadata.input),
  });

  return Object.assign(nodeFn, { type, Config, metadata } as const);
}

export interface ModuleMetadata<
  TInput extends TypeSpec = TypeSpec,
  TOutput extends TypeSpec = TypeSpec,
> {
  label: string;
  input: TInput;
  output: TOutput;
}
