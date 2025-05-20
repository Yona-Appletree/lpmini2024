import { expectTypeOf, test } from "vitest";
import { RecordDef } from "./record-def.tsx";
import { IntDef } from "./int-def.tsx";
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
