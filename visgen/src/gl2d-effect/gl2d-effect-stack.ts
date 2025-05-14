import type { Size2d } from "../util/size2d";
import type { EffectArgsFor, Gl2dEffectType } from "./gl2d-effect";

export interface Gl2dEffectStack {
  size: Size2d;
  items: StackItem[];
}

export function Gl2dEffectStack(args: Gl2dEffectStack): Gl2dEffectStack {
  return args;
}

/**
 * A stack of gl2d effects
 */
export interface StackItem {
  effectType: Gl2dEffectType;
  args: unknown;
}

export function StackItem<TType extends Gl2dEffectType>(
  effectType: TType,
  args: EffectArgsFor<TType>
): StackItem {
  return {
    effectType,
    args,
  };
}
