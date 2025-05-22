import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select.tsx";
import { defineType, type TypeMeta, TypeSpec } from "../type-spec.ts";
import { z } from "zod";

export const EnumDef = defineType(
  "enum",
  <TValues extends [string, ...string[]]>(
    meta: TypeMeta<TValues[number]> & {
      options: TValues;
    },
  ) =>
    TypeSpec(
      "enum",
      meta,
      z.enum(meta.options),
      ({ currentValue, onChange }) => {
        return (
          <div>
            <Select value={currentValue} onValueChange={onChange}>
              <SelectTrigger>
                <SelectValue placeholder="Select an option" />
              </SelectTrigger>
              <SelectContent>
                <SelectGroup>
                  {meta.options.map((option) => (
                    <SelectItem key={option} value={option}>
                      {option}
                    </SelectItem>
                  ))}
                </SelectGroup>
              </SelectContent>
            </Select>
          </div>
        );
      },
    ),
);
