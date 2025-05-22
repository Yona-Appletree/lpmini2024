import type { ConfigNodeProps } from "@/config/components/config-node.component.ts";
import type { RecordSpec } from "@/data/types/record-def.tsx";
import { ConfigNodeComponent } from "@/config/components/config-node.component.tsx";

export function RecordConfigComponent(
  props: ConfigNodeProps<Record<string, unknown>>,
) {
  const shape = props.typeSpec as RecordSpec;

  return (
    <div className="grid grid-cols-[auto_1fr] gap-2 p-1 items-baseline justify-items-start">
      {Object.entries(shape.info.meta.shape).map(([propName, valueSpec]) => {
        const value = props.value[propName];

        return (
          <>
            <label key={propName + "-label"} className="text-right">
              {propName}
            </label>
            <ConfigNodeComponent
              key={propName + "-value"}
              value={value}
              typeSpec={valueSpec}
              programConfig={props.programConfig}
              onChange={(value) => {
                // TODO Config mutability
                props.value[propName] = value;
              }}
            />
          </>
        );
      })}
    </div>
  );
}
