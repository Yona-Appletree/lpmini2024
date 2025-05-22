import { ArrayDef } from "./types/array/array-def.tsx";
import { EnumDef } from "./types/enum-def.tsx";
import { FloatDef } from "./types/float-def.tsx";
import { IntDef } from "./types/int-def.tsx";
import { RecordDef } from "./types/record-def.tsx";
import { Vec2Def } from "./types/array/vec2-def.tsx";
import { Vec3Def } from "./types/array/vec3-def.tsx";
import { Vec4Def } from "./types/array/vec4-def.tsx";

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
