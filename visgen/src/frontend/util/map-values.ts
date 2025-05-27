import type { AnyRecord } from "./types/any-record.ts";

export function mapValues<TIn extends AnyRecord, TOutValue>(
  obj: TIn,
  mapFn: (value: TIn[keyof TIn], key: keyof TIn) => TOutValue,
) {
  return Object.fromEntries(
    Object.entries(obj).map(([key, value]) => [
      key,
      mapFn(value as TIn[keyof TIn], key),
    ]),
  );
}
