export function glslTypeToRust(type: string): string {
  return typeMap[type] || type;
}

export const typeMap: { [key: string]: string } = {
  float: "f32",
  vec2: "Vec2Def",
  vec3: "Vec3Def",
  vec4: "Vec4Def",
  mat2: "Mat2",
  mat3: "Mat3",
  mat4: "Mat4",
};
