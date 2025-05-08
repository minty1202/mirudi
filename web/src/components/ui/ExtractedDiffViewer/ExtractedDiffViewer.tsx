import { ReactElement, useState } from "react";
import { DiffLine } from "./DiffLine";
import { DiffBlock } from "./DiffBlock";
import { diff, DiffResult } from "./helpers";
import { TabButton } from "./Tab";

interface DiffTableProps {
  diffResult: DiffResult;
}

function DiffTable({ diffResult }: DiffTableProps): ReactElement {

  if (!diffResult.diff || diffResult.diff.length === 0) {
    return <div className="text-sm text-gray-500">No differences found</div>;
  }

  return (
    <div className='border border-gray-300 rounded-md bg-white'>
      <table className="table-fixed w-full">
        <tbody>
          {(diffResult.type === "lines" || diffResult.type === "no-space-lines") && (
            <DiffLine diff={diffResult.diff} />
          )}
          {(diffResult.type === "words" || diffResult.type === "chars") && (
            <DiffBlock diff={diffResult.diff} />
          )}
        </tbody>
      </table>
    </div>
  );
}

type ViewType = "lines" | "no-space-lines" | "words" | "chars";
const tabs: ViewType[] = ["lines", "no-space-lines", "words", "chars"];

const diffFnMap: Record<ViewType, (data: { oldLines: string[]; newLines: string[] }) => DiffResult> = {
  lines: diff.lines,
  "no-space-lines": diff.noSpaceLines,
  words: diff.words,
  chars: diff.chars,
};

interface ExtractedDiffViewerProps {
  oldLines: string[];
  newLines: string[];
}

export function ExtractedDiffViewer({ oldLines, newLines }: ExtractedDiffViewerProps): ReactElement {
  const [tab, setTab] = useState<ViewType>("lines");

  const diffResult = diffFnMap[tab]({
    oldLines,
    newLines,
  });

  return (
    <>
      <div className="flex mb-2">
        {tabs.map((t) => (
          <TabButton key={t} selected={tab === t} onClick={() => setTab(t)}>
            {t}
          </TabButton>
        ))}
      </div>
      <DiffTable diffResult={diffResult} />
    </>
  )
}
