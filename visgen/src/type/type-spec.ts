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
  TMeta extends object,
  TSchema extends z.Schema,
>(name: TName, meta: TMeta, schema: TSchema) {
  return Object.assign(
    (input: z.input<TSchema>): z.output<TSchema> => schema.parse(input),
    {
      info: {
        name,
        meta,
      } as const,
      schema,
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
  TMeta extends object = object,
  TSchema extends z.Schema = z.Schema,
> {
  schema: TSchema;
  info: TypeInfo<TName, TMeta>;
}

export interface TypeSpecFn<
  TName extends string = string,
  TMeta extends object = object,
  TSchema extends z.Schema = z.Schema,
> extends TypeSpec<TName, TMeta, TSchema> {
  (...args: z.input<TSchema>): z.output<TSchema>;
}

export interface BaseTypeMeta {
  label?: string;
  description?: string;
}
