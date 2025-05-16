import { z, type ZodObject, type ZodRawShape } from "zod";

import type { ConfigEvalContext } from "./evaluate-config.ts";

export function defineConfigExpr<
  TType extends string,
  TShape extends ZodRawShape,
>(
  $expr: TType,
  shape: TShape,
  evalFn: (args: {
    context: ConfigEvalContext;
    value: z.output<ZodObject<TShape>>;
  }) => unknown,
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
    { type: $expr, schema, evalFn } as const,
  );
}
