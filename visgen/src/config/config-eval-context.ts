/**
 * Context in which configuration values are evaluated.
 */

export interface ConfigEvalContext {
  nodeMap: Map<
    string,
    {
      output: unknown;
    }
  >;
}
