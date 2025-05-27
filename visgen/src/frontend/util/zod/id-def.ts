import { z } from "zod";

export function IdDef<TBrand extends string>(type: TBrand) {
  const schema = z.string().brand(type);

  return Object.assign((idValue: string) => schema.parse(idValue), {
    type,
    schema,
  });
}
