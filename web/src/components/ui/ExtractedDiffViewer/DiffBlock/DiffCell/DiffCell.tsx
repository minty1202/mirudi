import { ReactElement } from "react";
import { DiffType } from "@/types";
import { tv } from "tailwind-variants";

const diffCellContent = tv({
  variants: {
    diffType: {
      added: "bg-green-200 text-green-900 mx-[1px] px-[2px]",
      removed: "bg-red-200 text-red-900 line-through mx-[1px] px-[2px]",
      replaced: "",
      equal: "",
    },
  },
});

interface DiffCellContentProps {
  content: string;
  diffType: DiffType;
}

function DiffCellContent({
  content,
  diffType,
}: DiffCellContentProps): ReactElement {
  return <span className={diffCellContent({ diffType })}>{content}</span>;
}

interface DiffCellProps {
  diff: DiffCellContentProps[];
}

export function DiffCell({ diff }: DiffCellProps): ReactElement {
  return (
    <>
      <td className="font-mono whitespace-pre-wrap text-sm align-top">
        <span className="px-2">
          {diff.map((item, index) => (
            <DiffCellContent key={index} {...item} />
          ))}
        </span>
      </td>
    </>
  );
}
