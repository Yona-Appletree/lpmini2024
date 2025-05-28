import { TypedObjectDef } from "@/frontend/util/zod/typed-object-def.ts";
import { configSchemaFor } from "../config/config-schema-for.ts";
import type { NodeInstance } from "./module-def.ts";
import type { TypeSpec } from "@/core/data/type-spec.ts";
import type { Gl2d } from "@/core/gl2d/gl2d.ts";

export function defineModule<TId extends string, TMeta extends ModuleMetadata>(
  type: TId,
  metadata: TMeta,
  nodeFn: (context: ModuleRuntimeContext) => NodeInstance<TMeta>,
) {
  const Config = TypedObjectDef(type, {
    input: configSchemaFor(metadata.input),
  });

  return Object.assign(nodeFn, { type, Config, metadata } as const);
}

export interface ModuleRuntimeContext {
  gl2d: Gl2d;
}

export interface ModuleMetadata<
  TInput extends TypeSpec = TypeSpec,
  TOutput extends TypeSpec = TypeSpec,
> {
  label: string;
  input: TInput;
  output: TOutput;
}
