import type { TypeSpec } from "@/data/type-spec.ts";
import { ArrayConfigComponent } from "@/config/components/array-config.component.tsx";
import { TupleConfigComponent } from "@/config/components/tuple-config.component.tsx";
import { configExprByType } from "@/config/config-expr.ts";
import { RecordConfigComponent } from "@/config/components/record-config.component.tsx";
import { isObject } from "@/util/is-object.ts";

export function ConfigNodeComponent(props: ConfigNodeProps) {
  switch (props.typeSpec.info.name) {
    case "record":
      return <RecordConfigComponent {...props} />;

    case "array":
      return <ArrayConfigComponent {...props} />;

    case "tuple":
      return <TupleConfigComponent {...props} />;

    default: {
      const value = props.value;

      if (isObject(value) && "$expr" in value) {
        const ConfigComponent = configExprByType[value["$expr"]].component;
        return <ConfigComponent {...props} />;
      } else {
        const InputComponent = props.typeSpec.component;
        return (
          <InputComponent
            meta={props.typeSpec.info.meta}
            currentValue={value}
            onChange={props.onChange}
          />
        );
      }
    }
  }
}

export interface ConfigNodeProps<T = unknown> {
  value: T;
  onChange: (value: T) => void;
  typeSpec: TypeSpec;
  programConfig: {
    nodes: Record<string, unknown>;
  };
}
