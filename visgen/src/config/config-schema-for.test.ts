import { z } from "zod";

import { describe, expect, expectTypeOf, test } from "vitest";
import { FloatDef } from "@/data/types/float-def.tsx";
import { configSchemaFor } from "./config-schema-for.ts";
import { Vec3Def } from "@/data/types/vec3-def.tsx";

describe("primitives", () => {
  test("number", () => {
    const type = FloatDef({ default: 0 });
    const schema = configSchemaFor(type);

    expectTypeOf<z.output<typeof schema>["value"]>().toEqualTypeOf<
      number | undefined
    >();

    expect(schema.parse({ value: 1 })).toEqual({ value: 1 });
  });
});

describe("tuples", () => {
  test("vec3", () => {
    const type = Vec3Def({ default: [0, 0, 0] });
    const schema = configSchemaFor(type);
    expectTypeOf<z.output<typeof schema>["value"]>().toEqualTypeOf<
      number | undefined
    >();

    expect(schema.parse({ value: [1, 2, 3] })).toEqual({ value: [1, 2, 3] });
  });
});
