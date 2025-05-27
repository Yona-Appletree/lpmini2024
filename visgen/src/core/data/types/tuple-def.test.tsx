/* eslint-disable unused-imports/no-unused-vars */
import { expectTypeOf } from "vitest";
import { TupleDef } from "@/core/data/types/tuple-def.tsx";
import { FloatDef } from "@/core/data/types/float-def.tsx";
import { z } from "zod";

expectTypeOf<TupleDef["info"]["name"]>().toEqualTypeOf<"tuple">();

const tuple2 = TupleDef([FloatDef({ default: 0 }), FloatDef({ default: 0 })]);

expectTypeOf<z.output<typeof tuple2.schema>>().toEqualTypeOf<
  [number, number]
>();
