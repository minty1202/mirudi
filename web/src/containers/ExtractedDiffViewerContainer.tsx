import { ReactElement, useState, useEffect } from "react";
import { BottomSheet } from "@/components/ui";
import { useSelectedDiffValue } from '@/contexts';
import { ExtractedDiffViewer } from "@/components/ui";

export function ExtractedDiffViewerContainer(): ReactElement {
  const [isOpen, setIsOpen] = useState(false);
  const { value } = useSelectedDiffValue();

  useEffect(() => {
    const leftData = value.left.data;
    const rightData = value.right.data;
    if (leftData.length > 0 && rightData.length > 0) {
      setIsOpen(true);
    } else {
      setIsOpen(false);
    }
  }, [value]);

  const oldLines = value.left.data.map((item) => item.content);
  const newLines = value.right.data.map((item) => item.content);

  return (
    <BottomSheet
      open={isOpen}
      onOpen={() => setIsOpen(true)}
      onClose={() => setIsOpen(false)}
    >
      <ExtractedDiffViewer oldLines={oldLines} newLines={newLines} />

  </BottomSheet>
  );
}
