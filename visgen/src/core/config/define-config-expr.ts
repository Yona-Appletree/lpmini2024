import { z } from "zod";

import type { ConfigEvalContext } from "./config-eval-context.ts";
import React from "react";

export function defineConfigExpr<
  TType extends string,
  TSchema extends z.Schema,
>(
  $expr: TType,
  meta: {
    icon?: string | React.JSX.Element;
    label: string;
  },
  schema: TSchema,
  evalFn: (args: {
    context: ConfigEvalContext;
    value?: z.output<TSchema>;
  }) => unknown,
  component: React.FunctionComponent<
    ConfigExprComponentProps<z.output<TSchema>>
  >
) {
  return Object.assign(
    (args: Omit<z.input<TSchema>, "$expr">) =>
      schema.parse({
        ...args,
        $expr,
      }),
    { exprKey: $expr, meta, schema, evalFn, component } as const
  );
}

export type ConfigExprComponentProps<T> = {
  exprValue: T | undefined;
  onChange: (value: T) => void;
  programConfig: {
    nodes: Record<string, unknown>;
  };
};

export type ConfigExprDef = ReturnType<
  typeof defineConfigExpr<string, z.Schema>
>;
