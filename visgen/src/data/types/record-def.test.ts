import { expectTypeOf, test } from "vitest";
import { RecordDef } from "./record-def.ts";
import { IntDef } from "./int-def.ts";
import { z } from "zod";

test("basic", () => {
  const TestRec = RecordDef({
    theta: IntDef({ default: 0 }),
  });

  expectTypeOf<z.output<typeof TestRec.schema>>().toEqualTypeOf<{
    theta: number;
  }>();

  TestRec({
    theta: 1,
  });
});
