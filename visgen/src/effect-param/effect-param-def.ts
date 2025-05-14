import { z, type ZodRawShape } from "zod";

export function EffectParamDef<
  TType extends string,
  TShape extends ZodRawShape,
>(type: TType, shape: TShape) {
  const fullSchema = z.object({
    ...shape,
    type: z.literal(type),
    label: z.string().optional(),
    description: z.string().optional(),
  });

  return Object.assign(
    (args: Omit<z.input<typeof fullSchema>, "type">) =>
      fullSchema.parse({
        ...args,
        type,
      }),
    { schema: fullSchema },
  );
}
