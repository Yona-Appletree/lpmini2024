import { z, type ZodRawShape } from "zod";

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
export function TypedObjectDef<
  TType extends string,
  TShape extends ZodRawShape,
>(type: TType, shape: TShape) {
  const schema = z.object({
    ...shape,
    type: z.literal(type),
  });

  return Object.assign(
    (args: Omit<z.input<typeof schema>, "type">) =>
      schema.parse({
        ...args,
        type,
      }),
    { schema },
  );
}
