import { z } from "zod";

export function GenericTypeDef<
  TSpecifier extends string,
  TFn extends (...args: any[]) => TypeDef<any>,
>(typeName: TSpecifier, typeFn: TFn) {
  return Object.assign(typeFn, {
    typeName: typeName,
  });
}

export function TypeDef<
  TSpecifier extends TypeSpecifier,
  TSchema extends z.Schema,
>(specifier: TSpecifier, schema: TSchema) {
  return Object.assign(
    (args: z.input<TSchema>): z.output<TSchema> => schema.parse(args),
    {
      schema,
      specifier,
    },
  );
}

export interface TypeMeta<T = unknown> {
  schema: z.Schema<T>;
  specifier: TypeSpecifier;
}

export interface TypeDef<T = unknown> extends TypeMeta<T> {
  (args: T): T;
}

export type TypeSpecifier = string | [string, SpecifierArg];
export type SpecifierArg =
  | string
  | SpecifierArg[]
  | { [key: string]: SpecifierArg };
