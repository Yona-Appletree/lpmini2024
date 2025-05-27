import type { ConfigNodeProps } from "@/core/config/components/config-node.component.tsx";
import type { TupleSpec } from "@/core/data/types/tuple-def.tsx";
import { ConfigNodeComponent } from "@/core/config/components/config-node.component.tsx";
import type { ConfigNode } from "@/core/config/config-node.ts";

export function TupleConfigComponent(props: ConfigNodeProps<ConfigNode[]>) {
  const typeSpec = props.typeSpec as TupleSpec;
  const value = props.configValue.value ?? [];

  return (
    <div className="flex flex-wrap gap-2">
      {typeSpec.info.meta.itemTypes.map((itemSpec, index) => {
        return (
          <ConfigNodeComponent
            key={index}
            typeSpec={itemSpec}
            configValue={value[index] ?? {}}
            programConfig={props.programConfig}
          />
        );
      })}
    </div>
  );
}
