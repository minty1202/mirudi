import { ViewerBox } from "./ViewerBox";

interface DiffViewerFallbacksProps {
  fileName: string;
}

export function DiffViewerLoading({ fileName }: DiffViewerFallbacksProps) {
  return (
    <ViewerBox fileName={fileName}>
      <div className="flex items-center justify-center h-32">
        <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-gray-900"></div>
      </div>
    </ViewerBox>
  );
}

interface DiffViewerErrorProps extends DiffViewerFallbacksProps {
  error: string;
}

export function DiffViewerError({ fileName, error }: DiffViewerErrorProps) {
  return (
    <ViewerBox fileName={fileName}>
      <div className="flex items-center justify-center h-32">
        <span className="text-red-500">{error}</span>
      </div>
    </ViewerBox>
  );
}

export function DiffViewerNoData({ fileName }: DiffViewerFallbacksProps) {
  return (
    <ViewerBox fileName={fileName}>
      <div className="flex items-center justify-center h-32">
        <span className="text-gray-500">No data available</span>
      </div>
    </ViewerBox>
  );
}
