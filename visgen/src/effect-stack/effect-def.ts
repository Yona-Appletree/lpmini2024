import type { EffectArguments } from "../effect-param/effect-arguments";
import type { EffectParams } from "../effect-param/effect-params";
import type { Gl2dContext } from "../gl2d/gl2d-context";
import {
  Gl2dFragmentShader,
  type ShaderUniformsRecord,
} from "../gl2d/gl2d-fragment-shader";
import { z } from "zod";
import { EffectParam } from "../effect-param/effect-param.ts";
import { Throw } from "../util/throw.ts";
import { TypedObjectDef } from "../util/zod/typed-object-def.ts";

export function EffectDef<TId extends string, TParams extends EffectParams>(
  type: TId,
  metadata: {
    label?: string;
    params: TParams;
  },
  glsl: string,
) {
  // Verify that the effect-param are valid
  const paramNameToUniformName: Record<string, string> = Object.fromEntries(
    Object.keys(metadata.params).map((key) => [
      key,
      "u" + key[0].toUpperCase() + key.slice(1),
    ]),
  );

  for (const [key, uniformName] of Object.entries(paramNameToUniformName)) {
    if (!glsl.includes(uniformName)) {
      throw new Error(
        `Uniform not found in shader: paramName=${key}, uniformName=${uniformName}`,
      );
    }
  }

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

  return Object.assign(
    (context: Gl2dContext) => {
      const shader = Gl2dFragmentShader(context, glsl);

      return {
        draw: (args: EffectArguments<TParams>) => {
          shader.draw(
            Object.fromEntries(
              Object.entries(paramNameToUniformName).map(
                ([paramName, uniformName]) => [
                  uniformName,
                  {
                    type: metadata.params[paramName].type,
                    value: args[paramName],
                  },
                ],
              ),
            ) as ShaderUniformsRecord,
          );
        },
      };
    },
    { type, Config, metadata, glsl } as const,
  );
}
