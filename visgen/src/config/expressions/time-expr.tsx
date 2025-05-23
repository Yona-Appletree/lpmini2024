import { defineConfigExpr } from "../define-config-expr.ts";
import { z } from "zod";
import { ClockIcon } from "lucide-react";
import { Input } from "@/components/ui/input.tsx";
import { useVersion } from "@/hooks/use-version.tsx";

export const TimeExpr = defineConfigExpr(
  "$time",
  {
    label: "Time Scaler",
    icon: <ClockIcon className="size-4" />,
  },
  z.object({
    scaleSeconds: z.number().default(1),
  }),
  ({ value }) => {
    // Get current time in seconds and multiply by the scaling factor
    const now = Date.now() / 1000; // Convert to seconds
    return now * (value?.scaleSeconds ?? 1);
  },
  (props) => {
    return (
      <div className="flex items-center gap-2">
        <Input
          type="number"
          className="w-[100px]"
          value={props.exprValue?.scaleSeconds}
          onChange={(e) => {
            props.onChange({
              scaleSeconds: parseFloat(e.target.value),
            });
          }}
          placeholder="Scale factor"
          step="0.1"
        />
        <span className="text-sm text-muted-foreground">seconds</span>
      </div>
    );
  }
);

export type TimeExpr = ReturnType<typeof TimeExpr>;
