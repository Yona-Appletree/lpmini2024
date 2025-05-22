import type { TypeSpec } from "@/data/type-spec.ts";
import { ArrayConfigComponent } from "@/config/components/array-config.component.tsx";
import { TupleConfigComponent } from "@/config/components/tuple-config.component.tsx";
import { RecordConfigComponent } from "@/config/components/record-config.component.tsx";
import { ValueConfigComponent } from "@/config/components/value-config-component.tsx";

import { configExprDefs } from "../config-expr";
import { RadioGroup, RadioGroupItem } from "@/components/ui/radio-group";
import { ButtonRadioGroupItem } from "@/components/button-radio-group";
import { ButtonRadioGroup } from "@/components/button-radio-group";

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
      <ButtonRadioGroup defaultValue="raw" onValueChange={props.onChange}>
        <ButtonRadioGroupItem value="raw" label="Raw" />
        {configExprDefs.map((it) => (
          <ButtonRadioGroupItem key={it.name} value={it.name} label={it.name} />
        ))}
      </ButtonRadioGroup>
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
