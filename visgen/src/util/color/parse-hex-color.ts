/**
 * Parses a hex color string into a normalized RGBA color.
 */
export function parseHexColor(hex: string): [number, number, number, number] {
  const [r, g, b] = hex.match(/\w\w/g)!.map((it) => parseInt(it, 16));
  return [r / 255, g / 255, b / 255, 1];
}
