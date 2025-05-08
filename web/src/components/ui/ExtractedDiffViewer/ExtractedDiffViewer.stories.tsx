import type { Meta, StoryObj } from "@storybook/react";
import { ExtractedDiffViewer } from "./ExtractedDiffViewer";

const meta: Meta<typeof ExtractedDiffViewer> = {
  title: "Components/UI/ExtractedDiffViewer",
  component: ExtractedDiffViewer,
  tags: ["autodocs"],
};

export default meta;


type Story = StoryObj<typeof ExtractedDiffViewer>;

export const Default: Story = {
  args: {
    oldLines: [
      "This is the first line",
      "This is the second line",
      "This is the third line",
    ],
    newLines: [
      "This is the first line",
      "This is the second line modified",
      "    This is the third line",
    ],
  },
}
