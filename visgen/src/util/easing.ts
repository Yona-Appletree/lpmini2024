/**
 * Validates that a number is in the normalized range (0-1)
 */
function normalize(t: number): number {
  return t % 1;
}

/**
 * Linear easing (no easing)
 */
export function easeLinear(t: number): number {
  return normalize(t);
}

/**
 * Sine easing
 */
export function easeSine(t: number): number {
  t = normalize(t);
  return 1 - Math.cos((t * Math.PI) / 2);
}

/**
 * Quadratic easing
 */
export function easeQuad(t: number): number {
  t = normalize(t);
  return t * t;
}

export function easeQuadIn(t: number): number {
  return easeQuad(t);
}

export function easeQuadOut(t: number): number {
  t = normalize(t);
  return 1 - (1 - t) * (1 - t);
}

export function easeQuadInOut(t: number): number {
  t = normalize(t);
  return t < 0.5 ? 2 * t * t : 1 - Math.pow(-2 * t + 2, 2) / 2;
}

/**
 * Cubic easing
 */
export function easeCubic(t: number): number {
  t = normalize(t);
  return t * t * t;
}

export function easeCubicIn(t: number): number {
  return easeCubic(t);
}

export function easeCubicOut(t: number): number {
  t = normalize(t);
  return 1 - Math.pow(1 - t, 3);
}

export function easeCubicInOut(t: number): number {
  t = normalize(t);
  return t < 0.5 ? 4 * t * t * t : 1 - Math.pow(-2 * t + 2, 3) / 2;
}

/**
 * Quartic easing
 */
export function easeQuart(t: number): number {
  t = normalize(t);
  return t * t * t * t;
}

export function easeQuartIn(t: number): number {
  return easeQuart(t);
}

export function easeQuartOut(t: number): number {
  t = normalize(t);
  return 1 - Math.pow(1 - t, 4);
}

export function easeQuartInOut(t: number): number {
  t = normalize(t);
  return t < 0.5 ? 8 * t * t * t * t : 1 - Math.pow(-2 * t + 2, 4) / 2;
}

/**
 * Quintic easing
 */
export function easeQuint(t: number): number {
  t = normalize(t);
  return t * t * t * t * t;
}

export function easeQuintIn(t: number): number {
  return easeQuint(t);
}

export function easeQuintOut(t: number): number {
  t = normalize(t);
  return 1 - Math.pow(1 - t, 5);
}

export function easeQuintInOut(t: number): number {
  t = normalize(t);
  return t < 0.5 ? 16 * t * t * t * t * t : 1 - Math.pow(-2 * t + 2, 5) / 2;
}

/**
 * Exponential easing
 */
export function easeExpo(t: number): number {
  t = normalize(t);
  return t === 0 ? 0 : Math.pow(2, 10 * t - 10);
}

export function easeExpoIn(t: number): number {
  return easeExpo(t);
}

export function easeExpoOut(t: number): number {
  t = normalize(t);
  return t === 1 ? 1 : 1 - Math.pow(2, -10 * t);
}

export function easeExpoInOut(t: number): number {
  t = normalize(t);
  if (t === 0) return 0;
  if (t === 1) return 1;
  if (t < 0.5) return Math.pow(2, 20 * t - 10) / 2;
  return (2 - Math.pow(2, -20 * t + 10)) / 2;
}

/**
 * Circular easing
 */
export function easeCirc(t: number): number {
  t = normalize(t);
  return 1 - Math.sqrt(1 - t * t);
}

export function easeCircIn(t: number): number {
  return easeCirc(t);
}

export function easeCircOut(t: number): number {
  t = normalize(t);
  return Math.sqrt(1 - Math.pow(t - 1, 2));
}

export function easeCircInOut(t: number): number {
  t = normalize(t);
  return t < 0.5
    ? (1 - Math.sqrt(1 - Math.pow(2 * t, 2))) / 2
    : (Math.sqrt(1 - Math.pow(-2 * t + 2, 2)) + 1) / 2;
}

/**
 * Back easing
 */
export function easeBack(t: number): number {
  t = normalize(t);
  const c1 = 1.70158;
  return c1 * t * t * t - c1 * t * t;
}

export function easeBackIn(t: number): number {
  return easeBack(t);
}

export function easeBackOut(t: number): number {
  t = normalize(t);
  const c1 = 1.70158;
  return 1 + c1 * Math.pow(t - 1, 3) + c1 * Math.pow(t - 1, 2);
}

export function easeBackInOut(t: number): number {
  t = normalize(t);
  const c2 = 2.5949095;
  return t < 0.5
    ? (Math.pow(2 * t, 2) * ((c2 + 1) * 2 * t - c2)) / 2
    : (Math.pow(2 * t - 2, 2) * ((c2 + 1) * (t * 2 - 2) + c2) + 2) / 2;
}

/**
 * Elastic easing
 */
export function easeElastic(t: number): number {
  t = normalize(t);
  const c4 = (2 * Math.PI) / 3;
  return t === 0
    ? 0
    : t === 1
      ? 1
      : -Math.pow(2, 10 * t - 10) * Math.sin((t * 10 - 10.75) * c4);
}

export function easeElasticIn(t: number): number {
  return easeElastic(t);
}

export function easeElasticOut(t: number): number {
  t = normalize(t);
  const c4 = (2 * Math.PI) / 3;
  return t === 0
    ? 0
    : t === 1
      ? 1
      : Math.pow(2, -10 * t) * Math.sin((t * 10 - 0.75) * c4) + 1;
}

export function easeElasticInOut(t: number): number {
  t = normalize(t);
  const c5 = (2 * Math.PI) / 4.5;
  return t === 0
    ? 0
    : t === 1
      ? 1
      : t < 0.5
        ? -(Math.pow(2, 20 * t - 10) * Math.sin((20 * t - 11.125) * c5)) / 2
        : (Math.pow(2, -20 * t + 10) * Math.sin((20 * t - 11.125) * c5)) / 2 +
          1;
}

/**
 * Bounce easing
 */
export function easeBounce(t: number): number {
  t = normalize(t);
  const n1 = 7.5625;
  const d1 = 2.75;
  if (t < 1 / d1) {
    return n1 * t * t;
  } else if (t < 2 / d1) {
    return n1 * (t -= 1.5 / d1) * t + 0.75;
  } else if (t < 2.5 / d1) {
    return n1 * (t -= 2.25 / d1) * t + 0.9375;
  } else {
    return n1 * (t -= 2.625 / d1) * t + 0.984375;
  }
}

export function easeBounceIn(t: number): number {
  t = normalize(t);
  return 1 - easeBounce(1 - t);
}

export function easeBounceOut(t: number): number {
  return easeBounce(t);
}

export function easeBounceInOut(t: number): number {
  t = normalize(t);
  return t < 0.5
    ? (1 - easeBounce(1 - 2 * t)) / 2
    : (1 + easeBounce(2 * t - 1)) / 2;
}

export const easingFunctions = {
  linear: easeLinear,
  sine: easeSine,
  quad: easeQuad,
  quadIn: easeQuadIn,
  quadOut: easeQuadOut,
  quadInOut: easeQuadInOut,
  cubic: easeCubic,
  cubicIn: easeCubicIn,
  cubicOut: easeCubicOut,
  cubicInOut: easeCubicInOut,
  quart: easeQuart,
  quartIn: easeQuartIn,
  quartOut: easeQuartOut,
  quartInOut: easeQuartInOut,
  quint: easeQuint,
  quintIn: easeQuintIn,
  quintOut: easeQuintOut,
  quintInOut: easeQuintInOut,
  expo: easeExpo,
  expoIn: easeExpoIn,
  expoOut: easeExpoOut,
  expoInOut: easeExpoInOut,
  circ: easeCirc,
  circIn: easeCircIn,
  circOut: easeCircOut,
  circInOut: easeCircInOut,
  back: easeBack,
  backIn: easeBackIn,
  backOut: easeBackOut,
  backInOut: easeBackInOut,
  elastic: easeElastic,
  elasticIn: easeElasticIn,
  elasticOut: easeElasticOut,
  elasticInOut: easeElasticInOut,
  bounce: easeBounce,
  bounceIn: easeBounceIn,
  bounceOut: easeBounceOut,
  bounceInOut: easeBounceInOut,
} as const;
export type EasingFn = (typeof easingFunctions)[keyof typeof easingFunctions];
export type EasingType = keyof typeof easingFunctions;
export const easingTypes = [
  "linear",
  "sine",
  "quad",
  "quadIn",
  "quadOut",
  "quadInOut",
  "cubic",
  "cubicIn",
  "cubicOut",
  "cubicInOut",
  "quart",
  "quartIn",
  "quartOut",
  "quartInOut",
  "quint",
  "quintIn",
  "quintOut",
  "quintInOut",
  "expo",
  "expoIn",
  "expoOut",
  "expoInOut",
  "circ",
  "circIn",
  "circOut",
  "circInOut",
  "back",
  "backIn",
  "backOut",
  "backInOut",
  "elastic",
  "elasticIn",
  "elasticOut",
  "elasticInOut",
  "bounce",
  "bounceIn",
  "bounceOut",
  "bounceInOut",
] as const;
