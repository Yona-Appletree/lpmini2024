import { expectTypeOf } from "vitest";
import type { RecordDef } from "./record-def.tsx";

expectTypeOf<RecordDef["info"]["name"]>().toEqualTypeOf<"record">();
