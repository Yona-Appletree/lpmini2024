import { z } from "zod";
import { expect, test } from "vitest";
import { TypedObjectDef } from "./typed-object-def.ts";

test("basic usage", () => {
  const Thing = TypedObjectDef("thing", { value: z.string() });
  const thing = Thing({ value: "hello" });
  expect(thing).toEqual({
    type: "thing",
    value: "hello",
  });
});
