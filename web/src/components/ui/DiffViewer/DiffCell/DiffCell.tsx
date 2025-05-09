import { ReactElement, memo } from "react";
import { tv } from "tailwind-variants";
import { DiffType } from "@/types";
import { useHighlighter } from "../contexts";
import styles from "./DiffCell.module.css";
import { bundledLanguages } from "shiki";
import { DiffIcon } from "@/components/ui/commons";
import { Highlighter } from "shiki";

interface CodeContentProps {
  code: string;
  lang: string;
  highlighter: Highlighter;
}

function CodeContent({
  code,
  lang,
  highlighter,
}: CodeContentProps): ReactElement {
  const normalizedLang = lang && lang in bundledLanguages ? lang : "plaintext";

  const highlightedCode = highlighter.codeToHtml(code || "", {
    lang: normalizedLang,
    theme: "github-light",
  });

  return (
    <span
      className={styles.inlineCode}
      dangerouslySetInnerHTML={{ __html: highlightedCode }}
    />
  );
}

const isSelected = {
  true: "after:inset-0 after:absolute after:bg-blue-400 after:mix-blend-multiply after:opacity-50",
  false: "",
};

const numberCell = tv({
  base: `relative text-sm w-[4ch] text-center align-top select-none`,
  variants: {
    diffType: {
      added: "bg-green-300",
      removed: "bg-red-300",
      replaced: "bg-yellow-300",
      equal: "",
      null: "bg-gray-100",
    },
    isSelected,
  },
  defaultVariants: {
    diffType: "equal",
    isSelected: false,
  },
});

const diffCell = tv({
  base: `relative text-sm w-[calc(50%-4ch)] align-top cursor-pointer select-none pr-2`,
  variants: {
    diffType: {
      added: "bg-green-100",
      removed: "bg-red-100",
      replaced: "bg-yellow-100",
      equal: "",
      null: "bg-gray-100",
    },
    isSelected,
  },
  defaultVariants: {
    diffType: "equal",
    isSelected: false,
  },
});

export interface DiffLine {
  lineNumber: number;
  content: string;
}

export interface DiffCellDataProps {
  value: DiffLine;
  lang: string;
  diffType: DiffType;
}

export interface DiffCellProps extends DiffCellDataProps {
  onMouseDown: (line: DiffLine) => void;
  onMouseEnter: (line: DiffLine) => void;
  selected?: boolean;
}

function DiffCell({
  value: { lineNumber, content },
  lang,
  diffType = "equal",
  onMouseDown,
  onMouseEnter,
  selected = false,
}: DiffCellProps): ReactElement {
  const highlighter = useHighlighter();

  return (
    <>
      <td className={numberCell({ diffType, isSelected: selected })}>
        {lineNumber}
      </td>
      <td
        className={diffCell({ diffType, isSelected: selected })}
        onMouseDown={() => onMouseDown({ lineNumber, content })}
        onMouseEnter={() => onMouseEnter({ lineNumber, content })}
      >
        <div className="flex items-start gap-1">
          <DiffIcon diffType={diffType} />
          {highlighter && (
            <CodeContent code={content} lang={lang} highlighter={highlighter} />
          )}
          {!highlighter && <span className="text-gray-500 font-mono">{content}</span>}
        </div>
      </td>
    </>
  );
}

export function EmptyCell(): ReactElement {
  return (
    <>
      <td className={numberCell({ diffType: "null" })}>&nbsp;</td>
      <td className={diffCell({ diffType: "null" })}>&nbsp;</td>
    </>
  );
}

const MemoizedDiffCell = memo(DiffCell, (prev, next) => {
  return (
    prev.selected === next.selected &&
    prev.value === next.value &&
    prev.lang === next.lang &&
    prev.diffType === next.diffType
  );
});

export { MemoizedDiffCell as DiffCell };
