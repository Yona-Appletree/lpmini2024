import type { ConfigNodeProps } from "@/config/components/config-node.component.ts";
import type { TupleSpec } from "@/data/types/tuple-def.tsx";
import { ConfigNodeComponent } from "@/config/components/config-node.component.tsx";
import type { ConfigNode } from "@/config/config-node.ts";

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
