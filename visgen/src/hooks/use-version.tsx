import { useState } from "react";

export function useVersion() {
  const [, setVersion] = useState(0);
  return () => setVersion((v) => v + 1);
}
