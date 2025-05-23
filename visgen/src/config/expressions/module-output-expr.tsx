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
import { z } from "zod";
import { CableIcon } from "lucide-react";

export const ModuleOutputExpr = defineConfigExpr(
  "$moduleOutput",
  {
    label: "Module Output",
    icon: <CableIcon className="size-4" />,
  },
  z.object({
    moduleId: z.string(),
  }),
  ({ context, value }) => {
    return value?.moduleId
      ? context.moduleMap.get(value.moduleId)?.output
      : undefined;
  },
  (props) => {
    return (
      <Select
        value={props.exprValue?.moduleId as string}
        onValueChange={(moduleId) =>
          props.onChange({ moduleId: moduleId as ModuleId })
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
