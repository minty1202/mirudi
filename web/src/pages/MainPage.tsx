import { ReactElement, useMemo } from "react";
import { DiffViewer } from "@/components/ui";
import { useFilesData } from "@/hooks";
import { extractExtension } from "@/utils";
import {
  DiffViewerContainer,
  ExtractedDiffViewerContainer,
} from "@/containers";
import { SelectedDiffValueProvider } from "@/contexts";

export function MainPage(): ReactElement {
  const { data, error, isLoading } = useFilesData();

  const extensions = useMemo(() => {
    if (!data) return [];
    const fileNames = Object.keys(data);
    if (!fileNames) return [];

    const extSet = new Set<string>();
    for (const filePath of fileNames) {
      const ext = extractExtension(filePath);
      if (ext) extSet.add(ext);
    }

    return Array.from(extSet).sort();
  }, [data]);

  if (isLoading) return <div>Loading...</div>;
  if (error) return <div>Error: {error.message}</div>;
  if (!data) return <div>No data</div>;

  return (
    <div className="min-h-screen">
      <SelectedDiffValueProvider>
        <DiffViewer.Provider extensions={extensions}>
          <div className="flex flex-col gap-4 p-4">
            {data.map((fileName, index) => (
              <DiffViewerContainer key={index} fileName={fileName} />
            ))}
          </div>
        </DiffViewer.Provider>
        <ExtractedDiffViewerContainer />
      </SelectedDiffValueProvider>
    </div>
  );
}
