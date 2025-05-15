import { z } from "zod";
import type { EffectParams } from "../effect-param/effect-params.ts";
import { EffectParam } from "../effect-param/effect-param.ts";
import { Throw } from "../util/throw.ts";
import { TypedObjectDef } from "../util/zod/typed-object-def.ts";

export function NodeDef<TId extends string, TParams extends EffectParams>(
  type: TId,
  metadata: {
    label?: string;
    params: TParams;
    output: EffectParam;
  },
) {
  const argsShape = Object.fromEntries(
    Object.entries(metadata.params).map(([key, param]) => [
      key,
      EffectParam.schemaRecord[param.type].shape.default ??
        Throw("Unknown param type: " + param.type),
    ]),
  ) as {
    [TKey in keyof TParams]: (typeof EffectParam.schemaRecord)[TParams[TKey]["type"]]["shape"]["default"];
  };

  const Config = TypedObjectDef(type, {
    args: z.object(argsShape),
  });

  return Object.assign(() => {}, { type, Config, metadata } as const);
}
