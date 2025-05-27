import { ZodDef } from "@/frontend/util/zod/zod-def.ts";
import { z } from "zod";
import { ModuleOutputExpr } from "./expressions/module-output-expr.tsx";
import { HexColorExpr } from "@/core/config/expressions/hex-color-expr.tsx";
import { TimeExpr } from "./expressions/time-expr.tsx";

// -----------------------------------------------------------------------------
// Expr definitions

export const configExprDefs = [
  ModuleOutputExpr,
  HexColorExpr,
  TimeExpr,
] as const;

// -----------------------------------------------------------------------------
// Map of exprKey to expr definition

export const configExprByType = Object.fromEntries(
  configExprDefs.map((def) => [def.exprKey, def]),
) as {
  [I in keyof typeof configExprDefs as (typeof configExprDefs)[I] extends {
    exprKey: string;
  }
    ? (typeof configExprDefs)[I]["exprKey"]
    : never]: (typeof configExprDefs)[I];
};

export type ConfigExprType = keyof typeof configExprByType;

// -----------------------------------------------------------------------------
// Config object keys ($moduleOutput, $literal, etc.)
//

export const ConfigExprKey = ZodDef(
  z.enum(configExprDefs.map((def) => def.exprKey) as unknown as ExprKeys),
);
export type ConfigExprKey = ReturnType<typeof ConfigExprKey>;

// -----------------------------------------------------------------------------

export const ConfigNodeExpr = ZodDef(
  z
    .object({
      ...(Object.fromEntries(
        configExprDefs.map((def) => [def.exprKey, def.schema]),
      ) as {
        [I in keyof typeof configExprDefs as (typeof configExprDefs)[I] extends {
          exprKey: string;
        }
          ? (typeof configExprDefs)[I]["exprKey"]
          : never]: (typeof configExprDefs)[I] extends {
          schema: infer TSchema;
        }
          ? TSchema
          : never;
      }),
    })
    .partial(),
);
export type ConfigNodeExpr = ReturnType<typeof ConfigNodeExpr>;

/**
 * ConfigNode expresses how to compute a value.
 *
 * It is an object with keys for each expression type, and a key "activeExpr"
 * that names the currently active expression.
 *
 * This structure allows the UI to retain the settings for each expression type,
 * if the user wants to change the expression type.
 *
 * Example:
 *
 * ```ts
 * {
 *   $moduleOutput: "myModule",
 *   value: "Hello, world!",
 *   activeExpr: "$moduleOutput",
 * }
 * ```
 */
export const ConfigNode = ZodDef(
  ConfigNodeExpr.schema.extend({
    value: z.unknown().optional(),
    activeExpr: ConfigExprKey.schema.optional(),
  }),
);

export type ConfigNode<T = unknown> = Omit<
  ReturnType<typeof ConfigNode>,
  "value"
> & {
  value?: T;
};

// -----------------------------------------------------------------------------

type ExprKeys<T extends readonly unknown[] = typeof configExprDefs> =
  T extends readonly [infer First, ...infer Rest]
    ? readonly [
        First extends { exprKey: string } ? First["exprKey"] : never,
        ...ExprKeys<Rest>,
      ]
    : readonly [];
