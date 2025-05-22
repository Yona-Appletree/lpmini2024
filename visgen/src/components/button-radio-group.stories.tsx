import type { Meta, StoryObj } from "@storybook/react";
import { ButtonRadioGroup, ButtonRadioGroupItem } from "./button-radio-group";
import { Image, FileText, Video } from "lucide-react";

const meta: Meta<typeof ButtonRadioGroup> = {
  title: "Components/ButtonRadioGroup",
  component: ButtonRadioGroup,
  tags: ["autodocs"],
};

export default meta;
type Story = StoryObj<typeof ButtonRadioGroup>;

export const Default: Story = {
  render: () => (
    <ButtonRadioGroup defaultValue="image">
      <ButtonRadioGroupItem
        value="image"
        icon={<Image className="h-8 w-8" />}
        label="Image"
      />
      <ButtonRadioGroupItem
        value="document"
        icon={<FileText className="h-8 w-8" />}
        label="Document"
      />
      <ButtonRadioGroupItem
        value="video"
        icon={<Video className="h-8 w-8" />}
        label="Video"
      />
    </ButtonRadioGroup>
  ),
};

export const Disabled: Story = {
  render: () => (
    <ButtonRadioGroup defaultValue="image">
      <ButtonRadioGroupItem
        value="image"
        icon={<Image className="h-8 w-8" />}
        label="Image"
        disabled
      />
      <ButtonRadioGroupItem
        value="document"
        icon={<FileText className="h-8 w-8" />}
        label="Document"
      />
      <ButtonRadioGroupItem
        value="video"
        icon={<Video className="h-8 w-8" />}
        label="Video"
      />
    </ButtonRadioGroup>
  ),
};

export const Vertical: Story = {
  render: () => (
    <ButtonRadioGroup defaultValue="image" className="flex-col">
      <ButtonRadioGroupItem
        value="image"
        icon={<Image className="h-8 w-8" />}
        label="Image"
      />
      <ButtonRadioGroupItem
        value="document"
        icon={<FileText className="h-8 w-8" />}
        label="Document"
      />
      <ButtonRadioGroupItem
        value="video"
        icon={<Video className="h-8 w-8" />}
        label="Video"
      />
    </ButtonRadioGroup>
  ),
};
