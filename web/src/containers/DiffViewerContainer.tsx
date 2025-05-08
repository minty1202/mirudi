import { ReactElement, memo } from "react";
import {
  DiffViewer,
  DiffViewerError,
  DiffViewerLoading,
  DiffViewerNoData,
} from "@/components/ui";
import { useSelectedDiffValue } from "@/contexts/SelectedDiffValueContext";
import { convertDiffViewData } from "@/utils";
import { useDiffData } from "@/hooks";

export type DiffViewerContainerProps = {
  fileName: string;
};

function rawDiffViewerContainer({
  fileName,
}: DiffViewerContainerProps): ReactElement {
  const { data, error, isLoading } = useDiffData(fileName);

  if (isLoading) return <DiffViewerLoading fileName={fileName} />;
  if (error) return <DiffViewerError fileName={fileName} error={error} />;
  if (!data) return <DiffViewerNoData fileName={fileName} />;

  const diffData = convertDiffViewData({
    fileName,
    diffData: data,
  });

  return <DiffViewerWithContext data={diffData} />;
}

const DiffViewerWithContext = ({
  data,
}: {
  data: ReturnType<typeof convertDiffViewData>;
}) => {
  const { value, onHover } = useSelectedDiffValue();
  return <DiffViewer data={data} value={value} onHover={onHover} />;
};

export const DiffViewerContainer = memo(
  rawDiffViewerContainer,
  (prevProps, nextProps) => {
    return prevProps.fileName === nextProps.fileName;
  },
);
