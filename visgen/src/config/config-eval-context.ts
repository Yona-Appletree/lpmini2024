/**
 * Context in which configuration values are evaluated.
 */

export interface ConfigEvalContext {
  moduleMap: Map<
    string,
    {
      output: unknown;
    }
  >;
}
