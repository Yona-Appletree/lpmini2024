import { z } from "zod";

/**
 * What is the unit of the vector? Unknown, normalized, etc.
 */
export const ValueUnit = z.enum(["unknown", "norm", "seconds"]);
export type ValueUnit = z.infer<typeof ValueUnit>;
