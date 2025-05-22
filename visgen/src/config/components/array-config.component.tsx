import type { ConfigNodeProps } from "@/config/components/config-node.component.ts";
import { ConfigNodeComponent } from "@/config/components/config-node.component.tsx";
import type { ArraySpec } from "@/data/types/array-def.tsx";
import type { ConfigNode } from "@/config/config-node.ts";

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
