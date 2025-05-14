import type { EffectArguments } from "../../params/effect-arguments";
import type { EffectParams } from "../../params/effect-params";
import type { Gl2dContext } from "../gl2d-context";

export function Gl2dEffect<TParams extends EffectParams>(
  params: TParams,
  factoryFn: (context: Gl2dContext) => {
    draw: (args: EffectArguments<TParams>) => void | Promise<void>;
  }
) {
  return Object.assign(factoryFn, { params });
}
