import type { ConfigNodeProps } from "@/config/components/config-node.component.ts";
import type { TupleSpec } from "@/data/types/tuple-def.tsx";
import { ConfigNodeComponent } from "@/config/components/config-node.component.tsx";

export function TupleConfigComponent(props: ConfigNodeProps<unknown[]>) {
  const typeSpec = props.typeSpec as TupleSpec;

  return (
    <div className="flex flex-wrap gap-2">
      {typeSpec.info.meta.itemTypes.map((itemSpec, index) => {
        return (
          <ConfigNodeComponent
            key={index}
            typeSpec={itemSpec}
            value={props.value[index]}
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
