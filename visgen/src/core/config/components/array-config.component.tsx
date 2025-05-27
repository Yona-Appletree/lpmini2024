import type { ConfigNodeProps } from "@/core/config/components/config-node.component.tsx";
import { ConfigNodeComponent } from "@/core/config/components/config-node.component.tsx";
import type { ArraySpec } from "@/core/data/types/array-def.tsx";
import type { ConfigNode } from "@/core/config/config-node.ts";

export function ArrayConfigComponent(props: ConfigNodeProps<ConfigNode[]>) {
  const typeSpec = props.typeSpec as ArraySpec;
  const itemSpec = typeSpec.info.meta.itemType;
  const value = props.configValue.value ?? [];

  return (
    <div className="flex flex-wrap gap-2">
      {value.map((itemValue, index) => {
        return (
          <ConfigNodeComponent
            key={index}
            typeSpec={itemSpec}
            configValue={itemValue}
            programConfig={props.programConfig}
          />
        );
      })}
    </div>
  );
}
