import type { Meta, StoryObj } from "@storybook/react";
import { ButtonRadioGroup, ButtonRadioGroupItem } from "./button-radio-group";
import {
  AlignCenter,
  AlignLeft,
  AlignRight,
  Bold,
  Italic,
  Underline,
} from "lucide-react";

const meta: Meta<typeof ButtonRadioGroup> = {
  title: "Components/ButtonRadioGroup",
  component: ButtonRadioGroup,
  tags: ["autodocs"],
};

export default meta;
type Story = StoryObj<typeof ButtonRadioGroup>;

export const TextAlignment: Story = {
  render: () => (
    <ButtonRadioGroup defaultValue="left">
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
  ),
};

export const TextFormatting: Story = {
  render: () => (
    <ButtonRadioGroup defaultValue="bold" className="w-fit">
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
  ),
};

export const IconOnly: Story = {
  render: () => (
    <ButtonRadioGroup defaultValue="left">
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
  ),
};

export const TextOnly: Story = {
  render: () => (
    <ButtonRadioGroup defaultValue="small">
      <ButtonRadioGroupItem value="small" label="Small" />
      <ButtonRadioGroupItem value="medium" label="Medium" />
      <ButtonRadioGroupItem value="large" label="Large" />
    </ButtonRadioGroup>
  ),
};

export const Disabled: Story = {
  render: () => (
    <ButtonRadioGroup defaultValue="left">
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
  ),
};
