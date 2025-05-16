import type { ShaderUniformsRecord } from "../gl2d/gl2d-fragment-shader";
import { useEffect, useRef } from "react";

import { Gl2dFragmentShader } from "../gl2d/gl2d-fragment-shader";
import { Gl2d } from "../gl2d/gl2d";
import { ImageDef } from "../type/types/image-def";
import { RecordDef } from "../type/types/record-def";
import { defineNode } from "./node-def";

export function Gl2dNodeDef<TId extends string, TParams extends RecordDef>(
  type: TId,
  metadata: {
    label: string;
    params: TParams;
  },
  glsl: string
) {
  // Verify that the effect-param are valid
  const paramNameToUniformName: Record<string, string> = Object.fromEntries(
    Object.keys(metadata.params.info.meta.shape).map((key) => [
      key,
      "u" + key[0].toUpperCase() + key.slice(1),
    ])
  );

  for (const [key, uniformName] of Object.entries(paramNameToUniformName)) {
    if (!glsl.includes(uniformName)) {
      throw new Error(
        `Uniform not found in shader: paramName=${key}, uniformName=${uniformName}`
      );
    }
  }

  return defineNode(
    type,
    {
      label: metadata.label,
      input: RecordDef({
        image: ImageDef(),
        args: metadata.params,
      }),
      output: ImageDef(),
    },
    () => {
      const gl2d = Gl2d();
      const shader = Gl2dFragmentShader(gl2d.context, glsl.trim());

      return {
        update: ({ input }) => {
          if (input.image) {
            gl2d.drawImage(input.image);
          }

          const args = Object.fromEntries(
            Object.entries(paramNameToUniformName).map(
              ([paramName, uniformName]) => [
                uniformName,
                {
                  type: metadata.params.info.meta.shape[paramName].info.name,
                  value: input.args?.[paramName as keyof typeof input.args],
                },
              ]
            )
          ) as ShaderUniformsRecord;
          shader.draw(args);
          gl2d.context.drawToScreen();
          gl2d.context.rotateFramebuffers();

          return gl2d.canvas;
        },
        component: ({ input }) => {
          const canvasRef = useRef<HTMLCanvasElement>(null);

          const ctx = canvasRef.current?.getContext("2d");
          if (ctx) {
            ctx.drawImage(gl2d.canvas, 0, 0);
          }

          return (
            <div>
              <canvas
                ref={canvasRef}
                width={gl2d.canvas.width}
                height={gl2d.canvas.height}
              />
            </div>
          );
        },
      };
    }
  );
}
