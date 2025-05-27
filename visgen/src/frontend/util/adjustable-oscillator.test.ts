import { expect, test } from "vitest";
import { AdjustableOscillator } from "@/frontend/util/adjustable-oscillator.ts";

test("initialize with the given period", () => {
  const oscillator = AdjustableOscillator({ period: 1000 });
  expect(oscillator.computeValue(0)).toBe(0);
  expect(oscillator.computeValue(500)).toBe(0.5);
  expect(oscillator.computeValue(1000)).toBe(0);
});

test("compute values correctly over time", () => {
  const oscillator = AdjustableOscillator({ period: 100 });
  expect(oscillator.computeValue(0)).toBe(0);
  expect(oscillator.computeValue(25)).toBe(0.25);
  expect(oscillator.computeValue(50)).toBe(0.5);
  expect(oscillator.computeValue(75)).toBe(0.75);
  expect(oscillator.computeValue(100)).toBe(0);
  expect(oscillator.computeValue(150)).toBe(0.5);
});

test("maintain value when period changes", () => {
  const oscillator = AdjustableOscillator({ period: 100 });

  // Get to 0.25 through the cycle
  const currentTime = 25;
  expect(oscillator.computeValue(currentTime)).toBeCloseTo(0.25);

  // Change to a period of 200
  oscillator.updatePeriod({ newPeriod: 200, currentTime });

  // Should still be at 0.25 through the cycle
  expect(oscillator.computeValue(currentTime)).toBeCloseTo(0.25);

  // Moving forward should maintain new period
  expect(oscillator.computeValue(currentTime + 50)).toBeCloseTo(0.5);
});

test("multiple period changes", () => {
  const oscillator = AdjustableOscillator({ period: 100 });

  // Start at 0.75 through cycle
  const time = 75;
  expect(oscillator.computeValue(time)).toBeCloseTo(0.75);

  // Change to period of 200
  oscillator.updatePeriod({ newPeriod: 200, currentTime: time });
  expect(oscillator.computeValue(time)).toBeCloseTo(0.75);

  // Change to period of 50
  oscillator.updatePeriod({ newPeriod: 50, currentTime: time });
  expect(oscillator.computeValue(time)).toBeCloseTo(0.75);
});

test("values beyond period", () => {
  const oscillator = AdjustableOscillator({ period: 100 });

  // Values should wrap around
  expect(oscillator.computeValue(150)).toBeCloseTo(0.5);
  expect(oscillator.computeValue(250)).toBeCloseTo(0.5);
  expect(oscillator.computeValue(375)).toBeCloseTo(0.75);
});
