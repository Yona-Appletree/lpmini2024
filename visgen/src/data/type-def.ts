import { ArrayDef } from "./types/array-def.ts";
import { EnumDef } from "./types/enum-def.ts";
import { FloatDef } from "./types/float-def.ts";
import { IntDef } from "./types/int-def.ts";
import { RecordDef } from "./types/record-def.ts";
import { Vec2Def } from "./types/vec2-def.ts";
import { Vec3Def } from "./types/vec3-def.ts";
import { Vec4Def } from "./types/vec4-def.ts";

export const TypeDef = [
  ArrayDef,
  EnumDef,
  FloatDef,
  IntDef,
  RecordDef,
  Vec2Def,
  Vec3Def,
  Vec4Def,
];

export type TypeName = (typeof TypeDef)[number]["name"];
