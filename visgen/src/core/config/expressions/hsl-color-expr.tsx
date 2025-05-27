import { defineConfigExpr } from "../define-config-expr.ts";

import { PaletteIcon } from "lucide-react";
import { configSchemaFor } from "@/core/config/config-schema-for.ts";
import { Vec3Def } from "@/core/data/types/vec3-def.tsx";
import { hslToRgba } from "@/frontend/util/color/hsl-to-rgba.ts";
import { evaluateConfig } from "@/core/config/evaluate-config.ts";
import { z } from "zod";

const hslType = Vec3Def({ default: [1, 0.5, 0.5] });

export const HslColorExpr = defineConfigExpr(
  "$hslColor",
  {
    label: "HSL Color",
    icon: <PaletteIcon className="size-4" />,
  },
  z.lazy(() => configSchemaFor(hslType)),
  ({ value, context }) => {
    const evaluated = evaluateConfig({
      context,
      configNode: value,
      path: [],
      spec: hslType,
    });
    return hslToRgba(evaluated[0], evaluated[1], evaluated[2]);
  },
  async (props) => {
    const { ConfigNodeComponent } = await import(
      "@/core/config/components/config-node.component.tsx"
    );

    return (
      <ConfigNodeComponent
        configValue={props.exprValue}
        typeSpec={hslType}
        programConfig={props.programConfig}
      />
    );
  },
);
export type HslColorExpr = ReturnType<typeof HslColorExpr>;
