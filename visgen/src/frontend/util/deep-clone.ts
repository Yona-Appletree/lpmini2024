/**
 * JSON-based deep clone. Can be improved later.
 * @param value
 */
export function deepClone<T>(value: T) {
  return JSON.parse(JSON.stringify(value));
}
