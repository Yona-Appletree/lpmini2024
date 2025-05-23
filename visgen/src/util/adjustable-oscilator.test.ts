import { describe, expect, it } from "vitest";
import { AdjustableOscilator } from "./adjustable-oscilator";

describe("AdjustableOscilator", () => {
  it("should initialize with the given period", () => {
    const oscillator = AdjustableOscilator({ period: 1000 });
    expect(oscillator.computeValue(0)).toBe(0);
    expect(oscillator.computeValue(500)).toBe(0.5);
    expect(oscillator.computeValue(1000)).toBe(0);
  });

  it("should compute values correctly over time", () => {
    const oscillator = AdjustableOscilator({ period: 100 });
    expect(oscillator.computeValue(0)).toBe(0);
    expect(oscillator.computeValue(25)).toBe(0.25);
    expect(oscillator.computeValue(50)).toBe(0.5);
    expect(oscillator.computeValue(75)).toBe(0.75);
    expect(oscillator.computeValue(100)).toBe(0);
    expect(oscillator.computeValue(150)).toBe(0.5);
  });

  it("should maintain value when period changes", () => {
    const oscillator = AdjustableOscilator({ period: 100 });

    // Get to 0.25 through the cycle
    const currentTime = 25;
    expect(oscillator.computeValue(currentTime)).toBeCloseTo(0.25, 5);

    // Change to a period of 200
    oscillator.updatePeriod({ newPeriod: 200, currentTime });

    // Should still be at 0.25 through the cycle
    expect(oscillator.computeValue(currentTime)).toBeCloseTo(0.25, 5);

    // Moving forward should maintain new period
    expect(oscillator.computeValue(currentTime + 50)).toBeCloseTo(0.5, 5);
  });

  it("should handle multiple period changes", () => {
    const oscillator = AdjustableOscilator({ period: 100 });

    // Start at time 75 in period 100 -> should be 0.75
    let time = 75;
    expect(oscillator.computeValue(time)).toBeCloseTo(0.75, 5);

    // Change to period 200, keeping time at 75
    oscillator.updatePeriod({ newPeriod: 200, currentTime: time });
    const valueAfterFirst = oscillator.computeValue(time);
    expect(valueAfterFirst).toBeCloseTo(0.75, 5);

    // Move time to 100 and change period to 50
    time = 100;
    oscillator.updatePeriod({ newPeriod: 50, currentTime: time });
    const finalValue = oscillator.computeValue(time);
    expect(finalValue).toBeCloseTo(0.75, 5);
  });

  it("should handle values beyond period", () => {
    const oscillator = AdjustableOscilator({ period: 100 });
    expect(oscillator.computeValue(150)).toBeCloseTo(0.5, 5);
    expect(oscillator.computeValue(250)).toBeCloseTo(0.5, 5);
    expect(oscillator.computeValue(375)).toBeCloseTo(0.75, 5);
  });

  // Additional test to verify value maintenance through time changes
  it("should maintain value when both time and period change", () => {
    const oscillator = AdjustableOscilator({ period: 100 });

    // Start at 0.75
    let time = 75;
    expect(oscillator.computeValue(time)).toBeCloseTo(0.75, 5);

    // Change period to 200, time stays at 75
    oscillator.updatePeriod({ newPeriod: 200, currentTime: time });
    expect(oscillator.computeValue(time)).toBeCloseTo(0.75, 5);

    // Change time to 100, verify value is still 0.75
    time = 100;
    expect(oscillator.computeValue(time)).toBeCloseTo(0.75, 5);

    // Change period to 50 at time 100
    oscillator.updatePeriod({ newPeriod: 50, currentTime: time });
    expect(oscillator.computeValue(time)).toBeCloseTo(0.75, 5);
  });
});
