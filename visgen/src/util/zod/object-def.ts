import { z } from "zod";

/**
 * Creates a factory function for json types backed by zod.
 *
 * Allows for a friendly style for building the data.
 *
 * ```typescript
 * const User = ObjectDef({ name: z.string() });
 * type User = ReturnType<typeof User>;
 *
 * const user = User({ name: 'John Doe' });
 *
 * console.info("User shape:", user.schema.shape);
 * ```
 */
export function ObjectDef<
  TShape extends z.ZodRawShape,
  TExtra extends object | undefined,
>(shape: TShape, extra?: TExtra) {
  const schema = z.object(shape);

  return Object.assign((args: z.input<typeof schema>) => schema.parse(args), {
    ...extra,
    schema,
  });
}
