import type { Meta, StoryObj } from "@storybook/react";
import { Button } from "./sample";

const meta: Meta<typeof Button> = {
  title: "Components/UI/Sample",
  component: Button,
  tags: ["autodocs"],
};

export default meta;

type Story = StoryObj<typeof Button>;

export const Default: Story = {
  args: {
    children: "Default Button",
  },
};

export const Outline: Story = {
  args: {
    children: "Outline Button",
    variant: "outline",
  },
};

export const Ghost: Story = {
  args: {
    children: "Ghost Button",
    variant: "ghost",
  },
};

export const Large: Story = {
  args: {
    children: "Large Button",
    size: "lg",
  },
};
