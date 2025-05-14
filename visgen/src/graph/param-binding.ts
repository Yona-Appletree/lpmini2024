export const LiteralArgument = z.object({
  argType: z.literal("literal"),
  value: z.unknown(),
});

export type LiteralArgument<T = unknown> = { value: T };

export const OutputRefArgument = z.object({
  argType: z.literal("output-ref"),
  nodeUid: z.string(),
  output: z.string(),
});

export type OutputRefArgument = z.infer<typeof OutputRefArgument>;

export const EffectArgument = z.discriminatedUnion("argType", [
  LiteralArgument,
  OutputRefArgument,
]);

export type EffectArgument<T = unknown> =
  | LiteralArgument<T>
  | OutputRefArgument;
