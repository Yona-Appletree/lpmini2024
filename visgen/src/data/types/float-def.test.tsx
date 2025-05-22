import { expect, expectTypeOf, test } from "vitest";
import { FloatDef } from "@/data/types/float-def.tsx";

expectTypeOf<FloatDef["info"]["name"]>().toEqualTypeOf<"float32">();

test("parsing", () => {
  const float = FloatDef({ default: 0 });
  expect(float.schema.parse(123)).toEqual(123);
});
