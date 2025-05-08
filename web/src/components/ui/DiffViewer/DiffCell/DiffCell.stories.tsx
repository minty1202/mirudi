import type { Meta, StoryObj } from "@storybook/react";
import { DiffCell, EmptyCell, DiffLine } from "./DiffCell";
import { HighlightProvider } from "../contexts";

const meta: Meta<typeof DiffCell> = {
  title: "Components/UI/DiffViewer/DiffCell",
  component: DiffCell,
  tags: ["autodocs"],
  decorators: [
    (Story) => {
      return (
        <HighlightProvider>
          <Story />
        </HighlightProvider>
      );
    },
  ],
  argTypes: {
    diffType: {
      control: { type: "radio" },
      options: ["added", "removed", "replaced", "equal"],
    },
  },
};

export default meta;

type Story = StoryObj<typeof DiffCell>;

const dummyProps = {
  onMouseDown: (line: DiffLine) => {
    console.log("mouseDown", line);
  },
  onMouseEnter: (line: DiffLine) => {
    console.log("mouseEnter", line);
  },
  selected: false,
};

export const Default: Story = {
  args: {
    value: {
      lineNumber: 1,
      content: "pub git: Arc<dyn GitWebProvider + Send + Sync>,",
    },
    lang: "rs",
    diffType: "added",
    ...dummyProps,
  },
};

export const Removed: Story = {
  args: {
    value: {
      lineNumber: 2,
      content: "pub git: Arc<dyn GitWebProvider + Send + Sync>,",
    },
    lang: "rs",
    diffType: "removed",
    ...dummyProps,
  },
};

export const Replaced: Story = {
  args: {
    value: {
      lineNumber: 3,
      content: "pub git: Arc<dyn GitWebProvider + Send + Sync>,",
    },
    lang: "rs",
    diffType: "replaced",
    ...dummyProps,
  },
};

export const Equal: Story = {
  args: {
    value: {
      lineNumber: 4,
      content: "pub git: Arc<dyn GitWebProvider + Send + Sync>,",
    },
    lang: "rs",
    diffType: "equal",
    ...dummyProps,
  },
};

export const Selected: Story = {
  args: {
    value: {
      lineNumber: 5,
      content: "pub git: Arc<dyn GitWebProvider + Send + Sync>,",
    },
    lang: "rs",
    diffType: "added",
    onMouseDown: () => {},
    onMouseEnter: () => {},
    selected: true,
  },
};

export const TsxAdded: Story = {
  args: {
    value: {
      lineNumber: 5,
      content: "<td className={numberCell({diffType})}>{lineNumber}</td>",
    },
    lang: "tsx",
    diffType: "added",
    ...dummyProps,
  },
};

export const Empty: StoryObj<typeof EmptyCell> = {
  render: () => <EmptyCell />,
};
