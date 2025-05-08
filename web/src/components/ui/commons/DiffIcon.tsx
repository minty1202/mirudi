import { tv } from "tailwind-variants";
import { DiffType } from "@/types";

const diffIcon = tv({
  base: "inline-flex items-center justify-center text-sm select-none px-1 font-mono",
  variants: {
    diffType: {
      added: "text-green-500",
      removed: "text-red-500",
      replaced: "text-yellow-500",
      equal: "",
    },
  },
  defaultVariants: {
    diffType: "equal",
  },
});

export function DiffIcon ({ diffType }: { diffType: DiffType }) {
  switch (diffType) {
    case "added":
      return <span className={diffIcon({ diffType })}>+</span>;
    case "removed":
      return <span className={diffIcon({ diffType })}>-</span>;
    case "replaced":
      return <span className={diffIcon({ diffType })}>~</span>;
    default:
      return <span className={diffIcon({ diffType })}>&nbsp;</span>;
  }
}
