import { z } from "zod";

import { describe, expect, expectTypeOf, test } from "vitest";
import { FloatDef } from "@/data/types/float-def.tsx";
import { configSchemaFor, type ConfigValueFor } from "./config-schema-for.ts";
import { Vec3Def } from "@/data/types/vec3-def.tsx";
import { RecordDef } from "@/data/types/record-def.tsx";

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

describe("records", () => {
  test("vec3", () => {
    const type = RecordDef({
      x: FloatDef({ default: 0 }),
      y: FloatDef({ default: 0 }),
    });
    const schema = configSchemaFor(type);

    expect(
      schema.parse({
        value: {
          x: { value: 1 },
          y: { value: 2 },
        },
      }),
    ).toEqual({
      value: {
        x: { value: 1 },
        y: { value: 2 },
      },
    });
  });
});

describe("tuples", () => {
  test("vec3", () => {
    const type = Vec3Def({ default: [0, 0, 0] });
    const schema = configSchemaFor(type);

    const config: ConfigValueFor<typeof type> = {
      value: [{ value: 1 }, { value: 1 }, { value: 1 }],
    } as const;

    expect(schema.parse(config)).toEqual(config);
  });
});
