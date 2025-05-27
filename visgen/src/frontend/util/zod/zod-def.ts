import { z } from "zod";

export function ZodDef<
  TSchema extends z.Schema,
  TExtra extends object | undefined,
>(schema: TSchema, extra?: TExtra) {
  return Object.assign(
    (args: z.input<TSchema>): z.output<TSchema> => schema.parse(args),
    {
      ...extra,
      schema,
    },
  );
}
