import { ReactElement, memo, JSXElementConstructor } from "react";
import type { Dispatch, SetStateAction } from "react";

import { DiffCell, EmptyCell } from "./DiffCell";
import type { DiffCellDataProps, DiffLine } from "./DiffCell";
import { useHoverSelect } from "./hooks";
import { HighlightProvider } from "./contexts";
import { ViewerBox } from "./ViewerBox";

export interface DiffRowProps {
  leftData?: DiffCellDataProps | null;
  rightData?: DiffCellDataProps | null;
}

export interface DiffViewData {
  fileName: string;
  diffData: DiffRowProps[];
}

export interface ExtractedDiffBlock {
  fileName: string;
  data: DiffLine[];
}

export interface ExtractedDiffBlockPair {
  left: ExtractedDiffBlock;
  right: ExtractedDiffBlock;
}

export interface DiffViewerProps {
  data: DiffViewData;
  value: ExtractedDiffBlockPair;
  onHover: Dispatch<SetStateAction<ExtractedDiffBlockPair>>;
}

function RawDiffViewer({
  data,
  value,
  onHover,
}: DiffViewerProps): ReactElement {
  const { left, right, onMouseUp } = useHoverSelect({
    data,
    value,
    onHover,
  });

  const { fileName, diffData } = data;

  const { selectionChecker: selectionCheckerLeft, ...leftRest } = left;
  const { selectionChecker: selectionCheckerRight, ...rightRest } = right;

  return (
    <>
      <ViewerBox fileName={fileName}>
        <table className="table-fixed w-full">
          <tbody onMouseUp={onMouseUp}>
            {diffData.map(({ leftData, rightData }, index) => (
              <tr key={index}>
                {leftData ? (
                  <DiffCell
                    {...leftData}
                    selected={selectionCheckerLeft(leftData.value.lineNumber)}
                    {...leftRest}
                  />
                ) : (
                  <EmptyCell />
                )}
                {rightData ? (
                  <DiffCell
                    {...rightData}
                    selected={selectionCheckerRight(rightData.value.lineNumber)}
                    {...rightRest}
                  />
                ) : (
                  <EmptyCell />
                )}
              </tr>
            ))}
          </tbody>
        </table>
      </ViewerBox>
    </>
  );
}

const DiffViewer = memo(RawDiffViewer) as unknown as ((
  props: DiffViewerProps,
) => ReactElement<unknown, string | JSXElementConstructor<DiffViewerProps>>) & {
  Provider: typeof HighlightProvider;
};

DiffViewer.Provider = HighlightProvider;

export { DiffViewer };
