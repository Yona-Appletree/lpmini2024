/**
 * Converts a normalized HSL color to an RGBA color.
 *
 * @param h - The hue of the color, normalized to 0-1.
 * @param s - The saturation of the color, normalized to 0-1.
 * @param l - The lightness of the color, normalized to 0-1.
 * @returns The RGBA color.
 */
export function hslToRgba(
  h: number,
  s: number,
  l: number
): [number, number, number, number] {
  const c = (1 - Math.abs(2 * l - 1)) * s;
  const x = c * (1 - Math.abs((((h * 360) / 60) % 2) - 1));
  const m = l - c / 2;
  const [r, g, b] = [
    [c, x, 0],
    [x, c, 0],
    [0, c, x],
  ][Math.floor((h * 360) / 60)];
  return [r + m, g + m, b + m, 1];
}
