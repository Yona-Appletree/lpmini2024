import { defineType, type TypeMeta, TypeSpec } from "../type-spec.ts";
import { z } from "zod";
import type { Gl2dTexture } from "@/core/gl2d/gl2d-texture.ts";

export const TextureDef = defineType(
  "texture",
  (meta: TypeMeta<Gl2dTexture | null> = { default: null }) =>
    TypeSpec(
      "texture",
      {
        glType: "texture",
        ...meta,
      },
      Gl2dTextureSchema.nullable(),
      () => {
        return <div>(null texture)</div>;
      },
    ),
);

export type TextureRgba = ReturnType<typeof TextureDef>;

export const Gl2dTextureSchema = z.custom<Gl2dTexture>(
  (it) => it["$type"] == "Gl2dTexture",
);
