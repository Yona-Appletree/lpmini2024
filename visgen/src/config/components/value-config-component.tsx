import { isObject } from "@/util/is-object.ts";
import { configExprByType } from "@/config/config-expr.ts";
import type { ConfigNodeProps } from "@/config/components/config-node.component.tsx";

export function ValueConfigComponent(props: ConfigNodeProps) {
  const value = props.configValue;

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
