import { expect, expectTypeOf, test } from "vitest";
import { RecordDef } from "./record-def.tsx";
import { FloatDef } from "./float-def.tsx";
import { IntDef } from "./int-def.tsx";
import { TypeValue } from "../type-spec.ts";

test("basic record", () => {
  expectTypeOf<RecordDef["info"]["name"]>().toEqualTypeOf<"record">();

  const record = RecordDef({
    float: FloatDef({ default: 0 }),
    int: IntDef({ default: 0 }),
  });

  expectTypeOf<TypeValue<typeof record>>().toEqualTypeOf<{
    float: number;
    int: number;
  }>();

  expect(record.schema.parse({ float: 1, int: 2 })).toEqual({
    float: 1,
    int: 2,
  });
});
