export function AdjustableOscillator(options: { period: number }) {
  let currentPeriod = options.period;
  let currentOffset = 0;

  const computeValue = (currentTime: number) => {
    const adjustedTime = currentTime + currentOffset;
    return (adjustedTime % currentPeriod) / currentPeriod;
  };

  const updatePeriod = ({
    newPeriod,
    currentTime,
  }: {
    newPeriod: number;
    currentTime: number;
  }) => {
    if (newPeriod === currentPeriod) {
      return;
    }

    const currentValue = computeValue(currentTime);
    currentPeriod = newPeriod;
    currentOffset = 0;

    const unadjustedNewValue = computeValue(currentTime);
    currentOffset = (currentValue - unadjustedNewValue) * newPeriod;
  };

  return {
    updatePeriod,
    computeValue,
  };
}
