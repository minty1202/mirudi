import { ReactElement, useMemo } from "react";
import { DiffViewer } from "@/components/ui";

import { useDiffData } from "@/hooks";
import { convertDiffViewData, extractExtension } from "@/utils";
import { DiffViewerContainer } from "@/containers/DiffViewerContainer";
import { SelectedDiffValueProvider } from "@/contexts";

export function MainPage(): ReactElement {
  const { data, error, isLoading } = useDiffData();

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

  const diffData = convertDiffViewData(data);

  return (
    <SelectedDiffValueProvider>
      <DiffViewer.Provider extensions={extensions}>
        {diffData.map((item, index) => (
          <DiffViewerContainer
            key={index}
            data={item}
          />
        ))}
      </DiffViewer.Provider>
    </SelectedDiffValueProvider>
  )
}
