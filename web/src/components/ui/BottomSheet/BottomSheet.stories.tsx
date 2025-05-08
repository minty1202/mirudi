import { useState } from 'react';
import type { Meta, StoryObj } from '@storybook/react';
import { BottomSheet } from './BottomSheet';

const meta: Meta<typeof BottomSheet> = {
  title: 'Components/UI/BottomSheet',
  component: BottomSheet,
  tags: ['autodocs'],
  argTypes: {
  },
  decorators: [
    (Story) => (
      <div style={{ height: '50vh', background: '#f0f0f0' }}>
        <Story />
      </div>
    ),
  ],
};

export default meta;

type Story = StoryObj<typeof BottomSheet>;

export const Default: Story = {
  render: () => {
    const [open, setOpen] = useState(false);

    return (
      <BottomSheet
        open={open}
        onOpen={() => setOpen(true)}
        onClose={() => setOpen(false)}
      >
        <div>BottomSheet Content</div>
      </BottomSheet>
    );
  },
};