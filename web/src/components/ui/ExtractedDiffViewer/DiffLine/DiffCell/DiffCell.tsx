import { ReactElement } from "react";
import { tv } from "tailwind-variants";
import { DiffType } from "@/types";
import { DiffIcon } from "@/components/ui/commons";

const diffCell = tv({
  base: `font-mono whitespace-pre-wrap text-sm align-top pr-2`,
  variants: {
    diffType: {
      added: "bg-green-100",
      removed: "bg-red-100",
      replaced: "bg-yellow-100",
      equal: "",
    },
  },
  defaultVariants: {
    diffType: "equal",
  },
});

export interface DiffCellProps {
  content: string;
  diffType: DiffType;
}

export function DiffCell({ content, diffType }: DiffCellProps): ReactElement {

  return (
    <>
      <td className={diffCell({diffType})}>
        <div className="flex items-start gap-1">
          <DiffIcon diffType={diffType} />
          <span>{content}</span>
        </div>
      </td>
    </>
  )
}
