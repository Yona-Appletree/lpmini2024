import { expectTypeOf, test } from "vitest";
import { RecordOf } from "./record-of.ts";
import { Int32 } from "./int32.ts";
import { z } from "zod";

test("basic", () => {
  const TestRec = RecordOf({
    theta: Int32(),
  });

  expectTypeOf<z.output<typeof TestRec.schema>>().toEqualTypeOf<{
    theta: number;
  }>();

  TestRec({
    theta: 1,
  });
});
