import { z } from "zod";
import { GenericTypeDef, TypeDef } from "../type-def.ts";

export const ArrayOf = GenericTypeDef("ArrayOf", (itemType: TypeDef) =>
  TypeDef(["ArrayOf", itemType.specifier], z.array(itemType.schema)),
);
