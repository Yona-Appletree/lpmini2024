import { z } from "zod";

/**
 * What kind of thing does this nodes represent?
 *
 * Not units, but the purpose of the nodes.
 */
export const ValueKind = z.enum([
  "unknown",
  "position",
  "dimension",
  "time",
  "color",
]);
export type ValueKind = z.infer<typeof ValueKind>;
