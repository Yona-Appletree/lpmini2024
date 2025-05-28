import type { ShaderUniformsRecord } from "../gl2d/gl2d-fragment-shader.ts";
import { Gl2dFragmentShader } from "../gl2d/gl2d-fragment-shader.ts";
import { useEffect, useRef } from "react";
import { TextureDef } from "@/core/data/types/texture-def.tsx";

import { defineModule } from "./define-module.ts";
import { type RecordSpec } from "@/core/data/types/record-def.tsx";
import type { RuntimeContext } from "@/core/program/program-runtime.ts";

export function defineGlModule<
  TId extends string,
  TMetadata extends {
    label: string;
    params: RecordSpec;
  },
>(type: TId, metadata: TMetadata, glsl: string) {
  // Verify that the effect-param are valid
  const paramNameToUniformName: Record<string, string> = Object.fromEntries(
    Object.keys(metadata.params.info.meta.shape).map((key) => [
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

  return defineModule(
    type,
    {
      label: metadata.label,
      input: metadata.params,
      output: TextureDef(),
    },
    ({ gl2d }) => {
      const shader = Gl2dFragmentShader(gl2d.context, glsl.trim());
      const framebuffer = gl2d.framebuffer();

      function Component(props: { context: RuntimeContext }) {
        const canvasRef = useRef<HTMLCanvasElement | null>(null);

        useEffect(() => {
          return props.context.addTickHandler(() => {
            const ctx = canvasRef.current?.getContext("2d");
            if (ctx) {
              framebuffer.texture.drawToScreen();
              ctx.drawImage(gl2d.canvas, 0, 0);
            }
          });
        }, [props.context]);

        return (
          <div>
            <canvas
              ref={canvasRef}
              width={gl2d.canvas.width}
              height={gl2d.canvas.height}
            />
          </div>
        );
      }

      return {
        update: ({ input }) => {
          const paramsShape = metadata.params.info.meta.shape;

          const args = Object.fromEntries(
            Object.entries(paramNameToUniformName).map(
              ([paramName, uniformName]) => [
                uniformName,
                {
                  type: paramsShape[paramName].info.meta.glType,
                  value: input?.[paramName as keyof typeof input],
                },
              ],
            ),
          ) as ShaderUniformsRecord;

          framebuffer.bind();
          shader.draw(args);

          return framebuffer.texture;
        },
        component: Component,
      };
    },
  );
}
