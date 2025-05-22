import { z } from "zod";
import type { RuntimeContext } from "@/program/program-runtime.ts";
import type { ShaderUniformValue } from "@/gl2d/gl2d-fragment-shader.ts";

export function defineType<
  TName extends string,
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  TFn extends (...args: any[]) => TypeSpecFn<TName, any, any>,
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
  component: TypeInputComponent<TMeta>,
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
    } as const,
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
  component: TypeInputComponent<TMeta>;
}

export type TypeInputComponent<TMeta extends TypeMeta<unknown>> =
  React.FunctionComponent<TypeInputComponentProps<TMeta>>;

export type TypeInputComponentProps<TMeta extends TypeMeta<unknown>> = {
  context: RuntimeContext;
  meta: TMeta;
  currentValue: TMeta["default"];
  onChange: (value: TMeta["default"]) => void;
};

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

export interface TypeMetaInfo {
  label?: string;
  description?: string;
  glType?: ShaderUniformValue["type"];
}

export interface TypeMeta<T> extends TypeMetaInfo {
  default: T;
}

export type TypeValue<T extends TypeSpec> = z.output<T["schema"]>;
