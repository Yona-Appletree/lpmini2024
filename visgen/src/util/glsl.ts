/**
 * Helper to allow glsl-literal plugin to recognize glsl code
 */
export function glsl(strings: TemplateStringsArray, ...values: any[]): string {
  return strings.reduce((result, str, i) => {
    return result + str + (values[i] ?? "");
  }, "");
}
