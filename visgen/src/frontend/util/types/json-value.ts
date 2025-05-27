export type JsonValue =
  | string
  | number
  | boolean
  | null
  | { [key: string]: JsonValue }
  | readonly [JsonValue, ...JsonValue[]]
  | JsonValue[];
