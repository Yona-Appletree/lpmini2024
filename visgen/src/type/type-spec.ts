import { z } from "zod";

export function GenericTypeDef<
  TName extends string,
  TArgs extends any[],
  TType extends TypeSpecFn<TName>,
  TFn extends (...args: TArgs) => TType,
>(typeName: TName, typeFn: TFn) {
  return Object.assign(typeFn, {
    typeName: typeName,
  });
}

export function TypeSpec<
  TName extends string,
  TSchema extends z.Schema,
  TMeta extends TypeMeta<z.output<TSchema>>,
>(name: TName, meta: TMeta, schema: TSchema) {
  return Object.assign(
    (input: z.input<TSchema>): z.output<TSchema> => schema.parse(input),
    {
      info: {
        name,
        meta,
      } as const,
      schema,
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
}

export interface TypeSpecFn<
  TName extends string = string,
  TSchema extends z.Schema = z.Schema,
  TMeta extends TypeMeta<z.output<TSchema>> = TypeMeta<z.output<TSchema>>,
> extends TypeSpec<TName, TSchema, TMeta> {
  (...args: z.input<TSchema>): z.output<TSchema>;
}

export type TypeSpecOf<T extends (...args: any[]) => TypeSpecFn> = {
  [K in keyof ReturnType<T>]: ReturnType<T>[K];
};

export interface TypeMeta<T> {
  label?: string;
  description?: string;
  default: T;
}

export type TypeValue<T extends TypeSpec> = z.output<T["schema"]>;
