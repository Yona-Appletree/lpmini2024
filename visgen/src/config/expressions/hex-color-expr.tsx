import { defineConfigExpr } from "../define-config-expr.ts";

import { z } from "zod";
import { PaletteIcon } from "lucide-react";
import { parseHexColor } from "@/util/color/parse-hex-color.ts";
import { Input } from "@/components/ui/input.tsx";

export const HexColorExpr = defineConfigExpr(
  "$hexColor",
  {
    label: "Color Picker",
    icon: <PaletteIcon className="size-4" />,
  },
  z.string(),
  ({ value }) => {
    return parseHexColor(value ?? "#AAAAFF");
  },
  (props) => {
    return (
      <Input
        type="color"
        className={"w-[72px]"}
        value={props.exprValue}
        onChange={(e) => {
          props.onChange(e.target.value);
        }}
      />
    );
  },
);
export type HexColorExpr = ReturnType<typeof HexColorExpr>;
