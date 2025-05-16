import { defineType, type TypeMeta, TypeSpec } from "../type-spec.ts";
import { z } from "zod";

export const ImageDef = defineType(
  "image",
  (meta: TypeMeta<CanvasImageSource | null> = { default: null }) =>
    TypeSpec("image", meta, CanvasImageSource.nullable()),
);

export type ImageRgba = ReturnType<typeof ImageDef>;

export const CanvasImageSource = z.custom<CanvasImageSource>(
  (it) =>
    it instanceof HTMLImageElement ||
    it instanceof SVGImageElement ||
    it instanceof HTMLVideoElement ||
    it instanceof HTMLCanvasElement ||
    it instanceof ImageBitmap ||
    it instanceof OffscreenCanvas ||
    it instanceof VideoFrame,
);
