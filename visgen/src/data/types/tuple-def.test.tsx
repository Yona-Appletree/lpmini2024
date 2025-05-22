/* eslint-disable unused-imports/no-unused-vars */
import { expectTypeOf, test } from "vitest";
import {
  TupleDef,
  type TupleItems,
  type TupleSchemas,
} from "@/data/types/tuple-def.tsx";
import { FloatDef } from "@/data/types/float-def.tsx";
import { z } from "zod";
import { TypeSpec } from "@/data/type-spec.ts";

expectTypeOf<TupleDef["info"]["name"]>().toEqualTypeOf<"tuple">();

const tuple2 = TupleDef([FloatDef({ default: 0 }), FloatDef({ default: 0 })]);

expectTypeOf(tuple2.schema).toEqualTypeOf<
  z.ZodTuple<[z.ZodNumber, z.ZodNumber]>
>();

test("TupleSchemas", () => {
  const floatType = FloatDef({ default: 0 }) satisfies TypeSpec;
  const twoFloats = [floatType, floatType] as const satisfies TupleItems;

  expectTypeOf<TupleSchemas<typeof twoFloats>>().toEqualTypeOf<
    [(typeof floatType)["schema"], (typeof floatType)["schema"]]
  >();

  const tupleSchema = z.tuple([
    floatType.schema,
    floatType.schema,
  ] as TupleSchemas<[typeof floatType, typeof floatType]>);

  expectTypeOf<z.output<typeof tupleSchema>>().toEqualTypeOf<
    [number, number]
  >();
});
