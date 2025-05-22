import type { ConfigNodeProps } from "@/config/components/config-node.component.ts";
import { ConfigNodeComponent } from "@/config/components/config-node.component.tsx";
import type { ArraySpec } from "@/data/types/array-def.tsx";

export function ArrayConfigComponent(props: ConfigNodeProps<unknown[]>) {
  const typeSpec = props.typeSpec as ArraySpec;
  const itemSpec = typeSpec.info.meta.itemType;

  return (
    <div className="flex flex-wrap gap-2">
      {props.configValue.map((value, index) => {
        return (
          <ConfigNodeComponent
            key={index}
            typeSpec={itemSpec}
            configValue={value}
            programConfig={props.programConfig}
            onChange={(value) => {
              // TODO Mutability
              props.configValue[index] = value;
            }}
          />
        );
      })}
    </div>
  );
}
