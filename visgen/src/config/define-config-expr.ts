import { z, type ZodObject, type ZodRawShape } from "zod";

import type { ConfigEvalContext } from "./config-eval-context.ts";

export function defineConfigExpr<
  TType extends string,
  TSchema extends z.Schema,
>(
  $expr: TType,
  schema: TSchema,
  evalFn: (args: {
    context: ConfigEvalContext;
    value: z.output<TSchema>;
  }) => unknown,
  component: React.FunctionComponent<{
    configValue: z.output<TSchema>;
    setValue: (value: z.output<TSchema>) => void;
    programConfig: {
      nodes: Record<string, unknown>;
    };
  }>
) {
  return Object.assign(
    (args: Omit<z.input<TSchema>, "$expr">) =>
      schema.parse({
        ...args,
        $expr,
      }),
    { exprKey: $expr, schema, evalFn, component } as const
  );
}

export type ConfigExprDef = ReturnType<
  typeof defineConfigExpr<string, z.Schema>
>;
