import type { TypeSpec } from "@/data/type-spec.ts";
import { ArrayConfigComponent } from "@/config/components/array-config.component.tsx";
import { TupleConfigComponent } from "@/config/components/tuple-config.component.tsx";
import { RecordConfigComponent } from "@/config/components/record-config.component.tsx";
import { ValueConfigComponent } from "@/config/components/value-config-component.tsx";

import { configExprDefs } from "../config-expr";
import { RadioGroup, RadioGroupItem } from "@/components/ui/radio-group";

export function ConfigNodeComponent(props: ConfigNodeProps) {
  const editComponent = (() => {
    switch (props.typeSpec.info.name) {
      case "record":
        return <RecordConfigComponent {...props} />;

      case "array":
        return <ArrayConfigComponent {...props} />;

      case "tuple":
        return <TupleConfigComponent {...props} />;

      default:
        return <ValueConfigComponent {...props} />;
    }
  })();

  return (
    <div>
      {editComponent}
      <RadioGroup>
        <RadioGroupItem value="raw">Raw</RadioGroupItem>
        {configExprDefs.map((it) => (
          <RadioGroupItem value={it.name}>{it.name}</RadioGroupItem>
        ))}
      </RadioGroup>
    </div>
  );
}

export interface ConfigNodeProps<T = unknown> {
  configValue: T;
  onChange: (value: T) => void;
  typeSpec: TypeSpec;
  programConfig: {
    nodes: Record<string, unknown>;
  };
}
