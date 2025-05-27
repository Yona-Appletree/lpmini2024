export interface Size2d {
  width: number;
  height: number;
}

export function Size2d(width: number, height: number): Size2d {
  return { width, height };
}
