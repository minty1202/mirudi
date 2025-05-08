import type { Meta, StoryObj } from "@storybook/react";
import { DiffBlock } from "./DiffBlock";

const meta: Meta<typeof DiffBlock> = {
  title: "Components/UI/ExtractedDiffViewer/DiffBlock",
  component: DiffBlock,
  tags: ["autodocs"],
};

export default meta;

type Story = StoryObj<typeof DiffBlock>;

export const Default: Story = {
  args: {
    diff: [
        [
          {
              "content": "        .route(\"/",
              "diffType": "equal"
          },
          {
              "content": "changes",
              "diffType": "removed"
          },
          {
              "content": "files",
              "diffType": "added"
          },
          {
              "content": "\", get(get_changed_files))",
              "diffType": "equal"
          }
      ]
    ],
  },
}
