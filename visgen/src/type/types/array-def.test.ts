import { expect, test } from "vitest";
import { IntDef } from "./int-def.ts";
import { ArrayDef } from "./array-def.ts";

test("basic", () => {
  const TestDef = ArrayDef(IntDef({ default: 0 }));

  // TODO Fix this broken type
  //expectTypeOf<z.output<typeof TestDef.schema>>().toEqualTypeOf<number[]>();

  const value = [1, 2, 3];
  expect(TestDef(value)).toEqual(value);
});
