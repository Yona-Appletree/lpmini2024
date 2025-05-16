import { z } from "zod";

export function IdDef<TBrand extends string>(type: TBrand) {
  const prefix = type + ":";
  const schema = z.string().startsWith(prefix).brand(type);

  return Object.assign((idValue: string) => schema.parse(prefix + idValue), {
    type,
    schema,
  });
}
