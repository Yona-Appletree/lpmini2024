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
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectTrigger,
} from "@/components/ui/select";
import { PencilLineIcon } from "lucide-react";

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
            currentValue={
              props.configValue.value ?? props.typeSpec.info.meta.default
            }
            onChange={(it) => {
              props.configValue.value = it;
            }}
          />
        );
      }
    }
  })();

  return (
    <div className="flex items-center">
      <Select
        value={activeExprKey ?? "value"}
        onValueChange={(newVal) => {
          props.configValue.activeExpr =
            newVal === "value" ? undefined : (newVal as ConfigExprKey);
        }}
      >
        <SelectTrigger showChevron={false}>
          {configExprDefs.find((it) => it.exprKey === activeExprKey)?.meta
            .icon ?? <PencilLineIcon className="w-4" />}
        </SelectTrigger>
        <SelectContent>
          <SelectGroup>
            <SelectItem key="value" value="value">
              <PencilLineIcon className="w-4" />
              Value
            </SelectItem>
            {configExprDefs.map((option) => (
              <SelectItem key={option.exprKey} value={option.exprKey}>
                {option.meta.icon}
                {option.meta.label}
              </SelectItem>
            ))}
          </SelectGroup>
        </SelectContent>
      </Select>
      {valueComponent}
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
