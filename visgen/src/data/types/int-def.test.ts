import { expect, expectTypeOf, test } from "vitest";
import { IntDef } from "./int-def.tsx";
import { z } from "zod";

test("basic", () => {
  const intType = IntDef({ default: 0 });

  expectTypeOf<z.output<typeof intType.schema>>().toEqualTypeOf<number>();

  const value = 10;
  expect(intType(value)).toEqual(value);
});
