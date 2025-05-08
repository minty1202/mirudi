import { ReactElement } from "react";
import { DiffCell } from "./DiffCell";
import { DiffContent } from "../helpers";

interface DiffLineProps {
  diff: DiffContent[];
}

export function DiffLine({ diff: diffLines }: DiffLineProps): ReactElement {
  return (
    <>
      {diffLines.map((diff, index) => (
        <tr key={index}>
          <DiffCell {...diff} />
        </tr>
      ))}
    </>
  );
}
