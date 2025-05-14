import { z } from "zod";

/**
 * What kind of thing does this value represent?
 *
 * Not units, but the purpose of the value.
 */
export const ValueKind = z.enum(["unknown", "position", "dimension", "color"]);
export type ValueKind = z.infer<typeof ValueKind>;
