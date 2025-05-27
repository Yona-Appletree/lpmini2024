import * as React from "react";
import * as RadioGroupPrimitive from "@radix-ui/react-radio-group";

//
// From https://github.com/shadcn-ui/ui/discussions/764
//

import { cn } from "../util/utils.ts";

const ButtonRadioGroup = React.forwardRef<
  React.ComponentRef<typeof RadioGroupPrimitive.Root>,
  React.ComponentPropsWithoutRef<typeof RadioGroupPrimitive.Root>
>(({ className, ...props }, ref) => {
  return (
    <RadioGroupPrimitive.Root
      className={cn("inline-flex gap-1 rounded-lg bg-muted p-1", className)}
      {...props}
      ref={ref}
    />
  );
});
ButtonRadioGroup.displayName = RadioGroupPrimitive.Root.displayName;

const ButtonRadioGroupItem = React.forwardRef<
  React.ElementRef<typeof RadioGroupPrimitive.Item>,
  {
    icon?: React.ReactNode;
    label: string;
  } & React.ComponentPropsWithoutRef<typeof RadioGroupPrimitive.Item>
>(({ className, icon, label, ...props }, ref) => {
  return (
    <RadioGroupPrimitive.Item
      ref={ref}
      className={cn(
        "inline-flex items-center justify-center gap-2 rounded-md p-1 text-sm font-medium ring-offset-background ",
        "transition-all focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring",
        " focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50",
        "hover:bg-background/50 ",
        "data-[state=checked]:bg-primary data-[state=checked]:text-primary-foreground",
        className,
      )}
      {...props}
    >
      {icon && <span className="h-4 w-4">{icon}</span>}
      {label && <span>{label}</span>}
    </RadioGroupPrimitive.Item>
  );
});
ButtonRadioGroupItem.displayName = RadioGroupPrimitive.Item.displayName;

export { ButtonRadioGroup, ButtonRadioGroupItem };
