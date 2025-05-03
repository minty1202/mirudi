import { ReactElement } from "react";
import { DiffViewer, DiffViewerProps } from "@/components/ui";
import { useSelectedDiffValue } from "@/contexts/SelectedDiffValueContext";

export type DiffViewerContainerProps = Omit<DiffViewerProps, "value" | "onHover">

export function DiffViewerContainer({
  data,
}: DiffViewerContainerProps): ReactElement {
  const { value, onHover } = useSelectedDiffValue();
  return (
    <DiffViewer
      data={data}
      value={value}
      onHover={onHover}
    />
  );
}
