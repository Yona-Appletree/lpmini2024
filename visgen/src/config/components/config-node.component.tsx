import type { TypeSpec } from "@/data/type-spec.ts";
import { ArrayConfigComponent } from "@/config/components/array-config.component.tsx";
import { TupleConfigComponent } from "@/config/components/tuple-config.component.tsx";
import { RecordConfigComponent } from "@/config/components/record-config.component.tsx";

import {
  configExprByType,
  configExprDefs,
  type ConfigExprKey,
  type ConfigNode,
} from "../config-node";
import {
  ButtonRadioGroup,
  ButtonRadioGroupItem,
} from "@/components/button-radio-group";

export function ConfigNodeComponent(props: ConfigNodeProps) {
  const activeExprKey = props.configValue.activeExpr;

  const valueComponent = (() => {
    // Active expression
    if (activeExprKey != null) {
      const ExprComponent = configExprByType[activeExprKey].component;
      return (
        <ExprComponent
          programConfig={props.programConfig}
          exprValue={props.configValue[activeExprKey]}
          onChange={(it) => (props.configValue[activeExprKey] = it)}
        />
      );
    }

    // Value
    switch (props.typeSpec.info.name) {
      case "record":
        return (
          <RecordConfigComponent
            {...(props as ConfigNodeProps<Record<string, ConfigNode>>)}
          />
        );

      case "array":
        return (
          <ArrayConfigComponent {...(props as ConfigNodeProps<ConfigNode[]>)} />
        );

      case "tuple":
        return (
          <TupleConfigComponent {...(props as ConfigNodeProps<ConfigNode[]>)} />
        );

      default: {
        const InputComponent = props.typeSpec.component;
        return (
          <InputComponent
            meta={props.typeSpec.info.meta}
            currentValue={props.configValue.value}
            onChange={(it) => (props.configValue.value = it)}
          />
        );
      }
    }
  })();

  return (
    <div>
      {valueComponent}

      <ButtonRadioGroup
        defaultValue="value"
        value={activeExprKey ?? "value"}
        onValueChange={(newVal) => {
          props.configValue.activeExpr =
            newVal === "value" ? undefined : (newVal as ConfigExprKey);
        }}
      >
        <ButtonRadioGroupItem value="value" label="Value" />
        {configExprDefs.map((it) => (
          <ButtonRadioGroupItem
            key={it.exprKey}
            value={it.exprKey}
            label={it.exprKey}
          />
        ))}
      </ButtonRadioGroup>
    </div>
  );
}

export interface ConfigNodeProps<T = unknown> {
  configValue: ConfigNode<T>;
  typeSpec: TypeSpec;
  programConfig: {
    nodes: Record<string, unknown>;
  };
}
