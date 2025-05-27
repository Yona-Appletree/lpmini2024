import { describe, expect, it } from "vitest";
import {
  easeBack,
  easeBackIn,
  easeBackInOut,
  easeBackOut,
  easeBounce,
  easeBounceIn,
  easeBounceInOut,
  easeBounceOut,
  easeCirc,
  easeCircIn,
  easeCircInOut,
  easeCircOut,
  easeCubic,
  easeCubicIn,
  easeCubicInOut,
  easeCubicOut,
  easeElastic,
  easeElasticIn,
  easeElasticInOut,
  easeElasticOut,
  easeExpo,
  easeExpoIn,
  easeExpoInOut,
  easeExpoOut,
  easeLinear,
  easeQuad,
  easeQuadIn,
  easeQuadInOut,
  easeQuadOut,
  easeQuart,
  easeQuartIn,
  easeQuartInOut,
  easeQuartOut,
  easeQuint,
  easeQuintIn,
  easeQuintInOut,
  easeQuintOut,
  easeSine,
} from "./easing.ts";

describe("easing functions", () => {
  // Test helper to check boundary conditions
  function testBoundaries(fn: (t: number) => number) {
    expect(fn(0)).toBe(0);
    expect(fn(1)).toBe(1);
    expect(() => fn(-0.1)).toThrow();
    expect(() => fn(1.1)).toThrow();
  }

  // Test helper to check monotonicity
  function testMonotonicity(fn: (t: number) => number) {
    const steps = 100;
    let prev = fn(0);
    for (let i = 1; i <= steps; i++) {
      const t = i / steps;
      const curr = fn(t);
      expect(curr).toBeGreaterThanOrEqual(prev);
      prev = curr;
    }
  }

  describe("linear", () => {
    it("should handle boundaries", () => {
      testBoundaries(easeLinear);
    });

    it("should be linear", () => {
      expect(easeLinear(0.5)).toBe(0.5);
    });
  });

  describe("sine", () => {
    it("should handle boundaries", () => {
      testBoundaries(easeSine);
    });

    it("should be monotonic", () => {
      testMonotonicity(easeSine);
    });
  });

  describe("quadratic", () => {
    it("should handle boundaries", () => {
      testBoundaries(easeQuad);
      testBoundaries(easeQuadIn);
      testBoundaries(easeQuadOut);
      testBoundaries(easeQuadInOut);
    });

    it("should be monotonic", () => {
      testMonotonicity(easeQuad);
      testMonotonicity(easeQuadIn);
      testMonotonicity(easeQuadOut);
      testMonotonicity(easeQuadInOut);
    });
  });

  describe("cubic", () => {
    it("should handle boundaries", () => {
      testBoundaries(easeCubic);
      testBoundaries(easeCubicIn);
      testBoundaries(easeCubicOut);
      testBoundaries(easeCubicInOut);
    });

    it("should be monotonic", () => {
      testMonotonicity(easeCubic);
      testMonotonicity(easeCubicIn);
      testMonotonicity(easeCubicOut);
      testMonotonicity(easeCubicInOut);
    });
  });

  describe("quartic", () => {
    it("should handle boundaries", () => {
      testBoundaries(easeQuart);
      testBoundaries(easeQuartIn);
      testBoundaries(easeQuartOut);
      testBoundaries(easeQuartInOut);
    });

    it("should be monotonic", () => {
      testMonotonicity(easeQuart);
      testMonotonicity(easeQuartIn);
      testMonotonicity(easeQuartOut);
      testMonotonicity(easeQuartInOut);
    });
  });

  describe("quintic", () => {
    it("should handle boundaries", () => {
      testBoundaries(easeQuint);
      testBoundaries(easeQuintIn);
      testBoundaries(easeQuintOut);
      testBoundaries(easeQuintInOut);
    });

    it("should be monotonic", () => {
      testMonotonicity(easeQuint);
      testMonotonicity(easeQuintIn);
      testMonotonicity(easeQuintOut);
      testMonotonicity(easeQuintInOut);
    });
  });

  describe("exponential", () => {
    it("should handle boundaries", () => {
      testBoundaries(easeExpo);
      testBoundaries(easeExpoIn);
      testBoundaries(easeExpoOut);
      testBoundaries(easeExpoInOut);
    });

    it("should be monotonic", () => {
      testMonotonicity(easeExpo);
      testMonotonicity(easeExpoIn);
      testMonotonicity(easeExpoOut);
      testMonotonicity(easeExpoInOut);
    });
  });

  describe("circular", () => {
    it("should handle boundaries", () => {
      testBoundaries(easeCirc);
      testBoundaries(easeCircIn);
      testBoundaries(easeCircOut);
      testBoundaries(easeCircInOut);
    });

    it("should be monotonic", () => {
      testMonotonicity(easeCirc);
      testMonotonicity(easeCircIn);
      testMonotonicity(easeCircOut);
      testMonotonicity(easeCircInOut);
    });
  });

  describe("back", () => {
    it("should handle boundaries", () => {
      testBoundaries(easeBack);
      testBoundaries(easeBackIn);
      testBoundaries(easeBackOut);
      testBoundaries(easeBackInOut);
    });

    it("should be monotonic", () => {
      testMonotonicity(easeBack);
      testMonotonicity(easeBackIn);
      testMonotonicity(easeBackOut);
      testMonotonicity(easeBackInOut);
    });
  });

  describe("elastic", () => {
    it("should handle boundaries", () => {
      testBoundaries(easeElastic);
      testBoundaries(easeElasticIn);
      testBoundaries(easeElasticOut);
      testBoundaries(easeElasticInOut);
    });

    // Note: Elastic functions are not monotonic by design
  });

  describe("bounce", () => {
    it("should handle boundaries", () => {
      testBoundaries(easeBounce);
      testBoundaries(easeBounceIn);
      testBoundaries(easeBounceOut);
      testBoundaries(easeBounceInOut);
    });

    // Note: Bounce functions are not monotonic by design
  });
});
