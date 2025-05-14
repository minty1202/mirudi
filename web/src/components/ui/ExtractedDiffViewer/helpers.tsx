import type { DiffType } from "@/types";
import {
  diffLines as diffLinesLib,
  diffWords as diffWordsLib,
  diffChars as diffCharsLib,
} from "diff";

import type { Change } from "diff";

type DiffFunction = (
  oldStr: string,
  newStr: string,
  options?: {
    ignoreCase?: boolean;
    ignoreWhitespace?: boolean;
    newlineIsToken?: boolean;
  },
) => Change[];

const mapDiffPartToType = (part: {
  added?: boolean;
  removed?: boolean;
}): DiffType => {
  if (part.added) return "added";
  if (part.removed) return "removed";
  return "equal";
};

type DiffContent = {
  content: string;
  diffType: DiffType;
};

type DiffLinesResult = {
  type: "lines";
  diff: DiffContent[];
};

type DiffBlockType = "words" | "chars";

type DiffBlocksResult = {
  type: DiffBlockType;
  diff: DiffContent[][];
};

type DiffResult = DiffLinesResult | DiffBlocksResult;

interface DiffDataPair {
  oldLines: string[];
  newLines: string[];
}

const diffLines = ({ oldLines, newLines }: DiffDataPair): DiffResult => {
  const oldBlock = oldLines.join("\n");
  const newBlock = newLines.join("\n");
  const diff = diffLinesLib(oldBlock, newBlock, {
    ignoreWhitespace: true,
  });
  const diffResult: DiffResult = {
    type: "lines",
    diff: [],
  };
  const lineDiff: DiffContent[] = [];
  diff.forEach((part) => {
    const diffType = mapDiffPartToType(part);
    const content = part.value || "";

    const splittedContent = content.split("\n");

    splittedContent.forEach((line) => {
      lineDiff.push({
        content: line,
        diffType,
      });
    });
  });
  diffResult.diff = lineDiff;
  return diffResult;
};

const createDiffBlocksFunction =
  (diffFn: DiffFunction, diffType: DiffBlockType) =>
  ({ oldLines, newLines }: DiffDataPair): DiffResult => {
    const oldBlock = oldLines.join("\n");
    const newBlock = newLines.join("\n");
    const diff = diffFn(oldBlock, newBlock);
    const diffResult: DiffResult = {
      type: diffType,
      diff: [],
    };

    const diffContentArray: DiffContent[][] = [];

    let currentLine: DiffContent[] = [];

    diff.forEach((part) => {
      const diffType = mapDiffPartToType(part);
      const content = part.value || "";

      const lines = content.split("\n");

      lines.forEach((line, index) => {
        if (line !== "") {
          currentLine.push({
            content: line,
            diffType,
          });
        }

        if (index < lines.length - 1) {
          diffContentArray.push(currentLine);
          currentLine = [];
        }
      });
    });

    if (currentLine.length > 0) {
      diffContentArray.push(currentLine);
    }

    diffResult.diff = diffContentArray;

    return diffResult;
  };

const diffWords = createDiffBlocksFunction(diffWordsLib, "words");

const diffChars = createDiffBlocksFunction(diffCharsLib, "chars");

export const diff = {
  lines: diffLines,
  words: diffWords,
  chars: diffChars,
};

export type { DiffResult, DiffContent };
