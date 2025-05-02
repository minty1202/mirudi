import { useRef } from 'react';
import { DiffLine } from "../DiffCell";
import { DiffViewerProps } from "../DiffViewer";
import { convertDiffBlock, extractSelectedLines } from '../helpers';

type SelectionAnchorState = {
  left: number | null;
  right: number | null;
};

export const useHoverSelect = ({
  data,
  value,
  onHover,
}: DiffViewerProps) => {
  const diffBlock = convertDiffBlock(data);

  const selectionAnchor = useRef<SelectionAnchorState>({
    left: null,
    right: null,
  });

  const handleMouseDown = (side: 'left' | 'right') => (diffLine: DiffLine) => {
    selectionAnchor.current[side] = diffLine.lineNumber;

    onHover(prev => ({
      ...prev,
      [side]: {
        ...prev[side],
        fileName: diffBlock[side].fileName,
        data: [diffLine],
      },
    }));
  };

  const handleMouseEnter = (side: 'left' | 'right') => ({ lineNumber: currentLine }: DiffLine) => {
    const startLine = selectionAnchor.current[side];
    if (startLine === null) return;

    const [start, end] = [startLine, currentLine].sort((a, b) => a - b);
    const lines = extractSelectedLines({ start, end }, diffBlock[side].data);

    onHover(prev => ({
      ...prev,
      [side]: {
        ...prev[side],
        fileName: diffBlock[side].fileName,
        data: lines,
      },
    }));
  };

  const handleMouseUp = () => {
    selectionAnchor.current = {
      left: null,
      right: null,
    };
  };

  const checkLineSelected = (side: 'left' | 'right') => (lineNumber: number): boolean => {
    const block = value[side];
    if (!block.data.length) return false;
    if (block.fileName !== diffBlock[side].fileName) return false;

    const selectedLines = value[side].data;
    const first = selectedLines[0]?.lineNumber;
    const last = selectedLines[selectedLines.length - 1]?.lineNumber;

    const [min, max] = [first, last].sort((a, b) => a - b);
    return lineNumber >= min && lineNumber <= max;
  };

  return {
    left: {
      onMouseDown: handleMouseDown('left'),
      onMouseEnter: handleMouseEnter('left'),
      selectionChecker: checkLineSelected('left'),
    },
    right: {
      onMouseDown: handleMouseDown('right'),
      onMouseEnter: handleMouseEnter('right'),
      selectionChecker: checkLineSelected('right'),
    },
    onMouseUp: handleMouseUp,
  };
};
