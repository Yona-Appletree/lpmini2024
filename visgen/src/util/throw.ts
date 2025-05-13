export function Throw(error: Error | string): never {
  if (error instanceof Error) {
    throw error;
  } else {
    throw new Error(error);
  }
}
