import type { Meta, StoryObj } from "@storybook/react";
import {
  ButtonRadioGroup,
  ButtonRadioGroupItem,
} from "./button-radio-group.tsx";
import {
  AlignCenter,
  AlignLeft,
  AlignRight,
  Bold,
  Italic,
  Underline,
} from "lucide-react";
import { useState } from "react";

const meta: Meta<typeof ButtonRadioGroup> = {
  title: "Components/ButtonRadioGroup",
  component: ButtonRadioGroup,
  tags: ["autodocs"],
};

export default meta;
type Story = StoryObj<typeof ButtonRadioGroup>;

const TextAlignmentDemo = () => {
  const [value, setValue] = useState("center");
  return (
    <div>
      <ButtonRadioGroup value={value} onValueChange={setValue}>
        <ButtonRadioGroupItem
          value="left"
          icon={<AlignLeft className="h-4 w-4" />}
          label="Left"
        />
        <ButtonRadioGroupItem
          value="center"
          icon={<AlignCenter className="h-4 w-4" />}
          label="Center"
        />
        <ButtonRadioGroupItem
          value="right"
          icon={<AlignRight className="h-4 w-4" />}
          label="Right"
        />
      </ButtonRadioGroup>
      <div className="text-sm text-muted-foreground">Selected: {value}</div>
    </div>
  );
};

export const TextAlignment: Story = {
  render: () => <TextAlignmentDemo />,
};

const TextFormattingDemo = () => {
  const [value, setValue] = useState("bold");
  return (
    <div>
      <ButtonRadioGroup
        value={value}
        onValueChange={setValue}
        className="w-fit"
      >
        <ButtonRadioGroupItem
          value="bold"
          icon={<Bold className="h-4 w-4" />}
          label="Bold"
        />
        <ButtonRadioGroupItem
          value="italic"
          icon={<Italic className="h-4 w-4" />}
          label="Italic"
        />
        <ButtonRadioGroupItem
          value="underline"
          icon={<Underline className="h-4 w-4" />}
          label="Underline"
        />
      </ButtonRadioGroup>
      <div className="text-sm text-muted-foreground">Selected: {value}</div>
    </div>
  );
};

export const TextFormatting: Story = {
  render: () => <TextFormattingDemo />,
};

const IconOnlyDemo = () => {
  const [value, setValue] = useState("center");
  return (
    <div>
      <ButtonRadioGroup value={value} onValueChange={setValue}>
        <ButtonRadioGroupItem
          value="left"
          icon={<AlignLeft className="h-4 w-4" />}
          label=""
          className="px-2"
        />
        <ButtonRadioGroupItem
          value="center"
          icon={<AlignCenter className="h-4 w-4" />}
          label=""
          className="px-2"
        />
        <ButtonRadioGroupItem
          value="right"
          icon={<AlignRight className="h-4 w-4" />}
          label=""
          className="px-2"
        />
      </ButtonRadioGroup>
      <div className="text-sm text-muted-foreground">Selected: {value}</div>
    </div>
  );
};

export const IconOnly: Story = {
  render: () => <IconOnlyDemo />,
};

const TextOnlyDemo = () => {
  const [value, setValue] = useState("small");
  return (
    <div className="">
      <ButtonRadioGroup value={value} onValueChange={setValue}>
        <ButtonRadioGroupItem value="small" label="Small" />
        <ButtonRadioGroupItem value="medium" label="Medium" />
        <ButtonRadioGroupItem value="large" label="Large" />
      </ButtonRadioGroup>
      <div className="text-sm text-muted-foreground">Selected: {value}</div>
    </div>
  );
};

export const TextOnly: Story = {
  render: () => <TextOnlyDemo />,
};

const DisabledDemo = () => {
  const [value, setValue] = useState("left");
  return (
    <div>
      <ButtonRadioGroup value={value} onValueChange={setValue}>
        <ButtonRadioGroupItem
          value="left"
          icon={<AlignLeft className="h-4 w-4" />}
          label="Left"
          disabled
        />
        <ButtonRadioGroupItem
          value="center"
          icon={<AlignCenter className="h-4 w-4" />}
          label="Center"
        />
        <ButtonRadioGroupItem
          value="right"
          icon={<AlignRight className="h-4 w-4" />}
          label="Right"
        />
      </ButtonRadioGroup>
      <div className="text-sm text-muted-foreground">Selected: {value}</div>
    </div>
  );
};

export const Disabled: Story = {
  render: () => <DisabledDemo />,
};
