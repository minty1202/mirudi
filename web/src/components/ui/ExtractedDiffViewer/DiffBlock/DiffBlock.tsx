import { ReactElement } from "react";
import { DiffCell } from "./DiffCell";
import { DiffContent } from "../helpers";

interface DiffBlockProps {
  diff: DiffContent[][];
}

export function DiffBlock({ diff: diffBlocks }: DiffBlockProps): ReactElement {
  return (
    <>
      {diffBlocks.map((diff, index) => (
        <tr key={index}>
          <DiffCell diff={diff} />
        </tr>
      ))}
    </>
  );
}
