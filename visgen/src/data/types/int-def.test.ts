import { expect, expectTypeOf, test } from "vitest";
import { IntDef } from "./int-def.ts";
import { z } from "zod";

test("basic", () => {
  const TestDef = IntDef({ default: 0 });

  expectTypeOf<z.output<typeof TestDef.schema>>().toEqualTypeOf<number>();

  const value = 10;
  expect(TestDef(value)).toEqual(value);
});
