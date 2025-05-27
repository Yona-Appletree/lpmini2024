import { z } from "zod";
import type { ShaderUniformValue } from "@/core/gl2d/gl2d-fragment-shader.ts";

export function defineType<
  TName extends string,
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  TFn extends (...args: any[]) => TypeSpecFn<TName, any, any>,
>(typeName: TName, typeFn: TFn) {
  return Object.assign(typeFn, {
    typeName,
  });
}

export function TypeSpec<
  TName extends string,
  TValue,
  TMeta extends TypeMeta<TValue>,
>(
  name: TName,
  meta: TMeta,
  schema: z.ZodSchema<TValue>,
  component: TypeInputComponent<TMeta>,
) {
  return Object.assign((input: TValue): TValue => schema.parse(input), {
    info: {
      name,
      meta,
    } as const,
    schema,

    // Typing the components is too hard right now.
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    component: component as TypeInputComponent<any>,
  } as const);
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
  TValue = unknown,
  TMeta extends TypeMeta<TValue> = TypeMeta<TValue>,
> {
  info: TypeInfo<TName, TMeta>;
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  component: TypeInputComponent<any>;
  schema: z.ZodSchema<TValue>;
}

export type TypeInputComponent<TMeta extends TypeMeta<unknown>> =
  React.FunctionComponent<TypeInputComponentProps<TMeta>>;

export type TypeInputComponentProps<TMeta extends TypeMeta<unknown>> = {
  meta: TMeta;
  currentValue: TMeta["default"];
  onChange: (value: TMeta["default"]) => void;
};

export interface TypeSpecFn<
  TName extends string = string,
  TValue = unknown,
  TMeta extends TypeMeta<TValue> = TypeMeta<TValue>,
> extends TypeSpec<TName, TValue, TMeta> {
  (args: TValue): TValue;
}

export type TypeSpecOf<
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  T extends (...args: any[]) => TypeSpecFn<any, any, any>,
> = {
  [K in keyof ReturnType<T>]: ReturnType<T>[K];
};

export interface TypeMetaInfo {
  label?: string;
  description?: string;
  glType?: ShaderUniformValue["type"];
}

export interface TypeMeta<T> extends TypeMetaInfo {
  default: T;
}

export type TypeValue<T extends TypeSpec> = T["info"]["meta"]["default"];
