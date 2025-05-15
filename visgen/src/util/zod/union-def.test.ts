import { expect, expectTypeOf, test } from "vitest";
import { z } from "zod";
import { TypedObjectDef } from "./typed-object-def.ts";
import { UnionDef } from "./union-def.ts";

const A = TypedObjectDef("a", {
  aProp: z.number(),
});
const B = TypedObjectDef("b", {
  bProp: z.number(),
});
const Union = UnionDef("type", [A.schema, B.schema]);

test("basic usage", () => {
  const a = Union("a", { aProp: 1 });
  const b = Union("b", { bProp: 2 });

  expect(a).toEqual({
    type: "a",
    aProp: 1,
  });

  expect(b).toEqual({
    type: "b",
    bProp: 2,
  });
});

test("schemaRecord", () => {
  expectTypeOf<(typeof Union.schemaRecord)["a"]>().toEqualTypeOf<
    typeof A.schema
  >();
  expectTypeOf<(typeof Union.schemaRecord)["b"]>().toEqualTypeOf<
    typeof B.schema
  >();

  expect(Union.schemaRecord.a).toEqual(A.schema);
  expect(Union.schemaRecord.b).toEqual(B.schema);
});
