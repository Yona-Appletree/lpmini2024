import { defineConfigExpr } from "../define-config-expr.ts";
import { ModuleId } from "@/program/module-id.ts";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select.tsx";

export const ModuleOutputExpr = defineConfigExpr(
  "node-output",
  {
    moduleId: ModuleId.schema,
  },
  ({ context, value }) => {
    return context.moduleMap.get(value.moduleId)?.output;
  },
  (props) => {
    return (
      <Select
        value={props.value.moduleId as string}
        onValueChange={(moduleId) =>
          props.setValue({ moduleId: moduleId as ModuleId })
        }
      >
        <SelectTrigger>
          <SelectValue placeholder="Select an option" />
        </SelectTrigger>
        <SelectContent>
          <SelectGroup>
            {Object.keys(props.programConfig.nodes).map((moduleId) => (
              <SelectItem key={moduleId} value={moduleId}>
                {moduleId}
              </SelectItem>
            ))}
          </SelectGroup>
        </SelectContent>
      </Select>
    );
  },
);
export type ModuleOutputExpr = ReturnType<typeof ModuleOutputExpr>;
