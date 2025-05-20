import { z } from "zod";
import { Component } from "react";
import type { DataInputComponent } from "./data-input-component";
import type { ConfigEvalContext } from "@/config/config-eval-context";
import type { RuntimeContext } from "@/graph/graph-runtime";

export function defineType<
  TName extends string,
  TType extends TypeSpecFn<TName>,
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  TFn extends (...args: any[]) => TType,
>(typeName: TName, typeFn: TFn) {
  return Object.assign(typeFn, {
    typeName: typeName,
  });
}

export function TypeSpec<
  TName extends string,
  TSchema extends z.Schema,
  TMeta extends TypeMeta<z.output<TSchema>>,
>(
  name: TName,
  meta: TMeta,
  schema: TSchema,
  component: TypeInputComponent<TSchema, TMeta>
) {
  return Object.assign(
    (input: z.input<TSchema>): z.output<TSchema> => schema.parse(input),
    {
      info: {
        name,
        meta,
      } as const,
      schema,
      component,
    } as const
  );
}

export interface TypeInfo<
  TName extends string = string,
  TMeta extends object = object,
> {
  name: TName;
  meta: TMeta;
}

export interface TypeSpec<
  TName extends string = string,
  TSchema extends z.Schema = z.Schema,
  TMeta extends TypeMeta<z.output<TSchema>> = TypeMeta<z.output<TSchema>>,
> {
  schema: TSchema;
  info: TypeInfo<TName, TMeta>;
  component: TypeInputComponent<TSchema, TMeta>;
}

export type TypeInputComponent<
  TSchema extends z.Schema = z.Schema,
  TMeta extends TypeMeta<z.output<TSchema>> = TypeMeta<z.output<TSchema>>,
> = React.FunctionComponent<{
  context: RuntimeContext;
  meta: TMeta;
  currentValue: z.output<TSchema>;
  onChange: (value: z.output<TSchema>) => void;
}>;

export interface TypeSpecFn<
  TName extends string = string,
  TSchema extends z.Schema = z.Schema,
  TMeta extends TypeMeta<z.output<TSchema>> = TypeMeta<z.output<TSchema>>,
> extends TypeSpec<TName, TSchema, TMeta> {
  (...args: z.input<TSchema>): z.output<TSchema>;
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export type TypeSpecOf<T extends (...args: any[]) => TypeSpecFn> = {
  [K in keyof ReturnType<T>]: ReturnType<T>[K];
};

export interface TypeMeta<T> {
  label?: string;
  description?: string;
  default: T;
}

export type TypeValue<T extends TypeSpec> = z.output<T["schema"]>;
