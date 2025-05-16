import { z, type ZodRawShape } from "zod";

export function ValueDef<TType extends string, TShape extends ZodRawShape>(
  $expr: TType,
  shape: TShape,
) {
  const schema = z.object({
    ...shape,
    $expr: z.literal($expr),
  });

  return Object.assign(
    (args: Omit<z.input<typeof schema>, "$expr">) =>
      schema.parse({
        ...args,
        $expr,
      }),
    { schema },
  );
}
