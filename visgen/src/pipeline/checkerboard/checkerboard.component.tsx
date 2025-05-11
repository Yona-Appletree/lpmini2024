import type { CheckerboardConfig } from "./checkerboard.stage";

export function CheckerboardComponent(props: { config: CheckerboardConfig }) {
  return (
    <div>
      <h1>{props.config.name}</h1>
    </div>
  );
}
