import { expectTypeOf, test } from "vitest";
import { IntDef } from "./int-def.ts";
import { z } from "zod";
import { ArrayDef } from "./array-def.ts";

test("basic", () => {
  const TestArray = ArrayDef(IntDef());

  expectTypeOf<z.output<typeof TestArray.schema>>().toEqualTypeOf<number[]>();

  TestArray([1, 2, 3]);
});
