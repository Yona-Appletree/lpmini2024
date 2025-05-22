import type { ConfigNodeProps } from "@/config/components/config-node.component.ts";
import { ConfigNodeComponent } from "@/config/components/config-node.component.tsx";
import type { ArraySpec } from "@/data/types/array-def.tsx";

export function ArrayConfigComponent(props: ConfigNodeProps<unknown[]>) {
  const typeSpec = props.typeSpec as ArraySpec;
  const itemSpec = typeSpec.info.meta.itemType;

  return (
    <div className="flex flex-wrap gap-2">
      {props.value.map((value, index) => {
        return (
          <ConfigNodeComponent
            key={index}
            typeSpec={itemSpec}
            value={value}
            programConfig={props.programConfig}
            onChange={(value) => {
              // TODO Mutability
              props.value[index] = value;
            }}
          />
        );
      })}
    </div>
  );
}
