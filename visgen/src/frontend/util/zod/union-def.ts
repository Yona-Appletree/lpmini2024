import { z, type ZodDiscriminatedUnionOption, ZodLiteral } from "zod";

export function UnionDef<
  TTypeProp extends string,
  TTypes extends readonly [
    ZodDiscriminatedUnionOption<TTypeProp>,
    ...ZodDiscriminatedUnionOption<TTypeProp>[],
  ],
>(typeProp: TTypeProp, options: TTypes) {
  const schema = z.discriminatedUnion(typeProp, options);

  type TDiscriminatorValue<TSchema = TTypes[number]> =
    TSchema extends ZodDiscriminatedUnionOption<TTypeProp>
      ? TSchema["shape"][TTypeProp] extends ZodLiteral<unknown>
        ? TSchema["shape"][TTypeProp]["value"] extends string
          ? TSchema["shape"][TTypeProp]["value"]
          : never
        : never
      : never;

  return Object.assign(
    <TType extends TDiscriminatorValue>(
      type: TType,
      args: Omit<z.input<SchemaForKey<TTypeProp, TType, TTypes>>, TTypeProp>,
    ) =>
      schema.parse({
        ...args,
        [typeProp]: type,
      }),
    {
      schema,
      schemaRecord: Object.fromEntries(
        options.map((option) => {
          const type = (
            option.shape[typeProp] as unknown as ZodLiteral<unknown>
          ).value;
          return [type, option];
        }),
      ) as {
        [TType in TTypes[number] as TDiscriminatorValue<TType>]: TType;
      },
    },
  );
}

// recursively find the schema for a given key
type SchemaForKey<TTypeProp extends string, TType, TTypes> = TTypes extends [
  infer TFirst,
  ...infer TRest,
]
  ? TFirst extends ZodDiscriminatedUnionOption<TTypeProp>
    ? TFirst["shape"][TTypeProp] extends ZodLiteral<TType>
      ? TType extends TFirst["shape"][TTypeProp]["value"]
        ? TFirst
        : SchemaForKey<TTypeProp, TType, TRest>
      : SchemaForKey<TTypeProp, TType, TRest>
    : SchemaForKey<TTypeProp, TType, TRest>
  : never;
