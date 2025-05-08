import type { Meta, StoryObj } from "@storybook/react";
import { DiffLine } from "./DiffLine";

const meta: Meta<typeof DiffLine> = {
  title: "Components/UI/ExtractedDiffViewer/DiffLine",
  component: DiffLine,
  tags: ["autodocs"],
};

export default meta;

type Story = StoryObj<typeof DiffLine>;

export const Default: Story = {
  args: {
    diff: [
      {
        content: "import { Button } from './components/ui';",
        diffType: "equal",
      },
      {
        content: "import { Button } from './components/ui';",
        diffType: "added",
      },
      {
        content: "import { Button } from './components/ui';",
        diffType: "removed",
      },
    ]
  },
}
