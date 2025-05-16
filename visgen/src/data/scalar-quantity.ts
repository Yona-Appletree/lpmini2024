export type UnknownQuantity = "unknown";
export function Distance(unit: "norm" | "pixel" | "meter") {
  return ["position", unit] as const;
}

export type ScalarQuantity = UnknownQuantity | ReturnType<typeof Distance>;
