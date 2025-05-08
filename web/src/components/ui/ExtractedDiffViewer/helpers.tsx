import type { DiffType } from "@/types";
import {
  diffLines as diffLinesLib,
  diffWords as diffWordsLib,
  diffChars as diffCharsLib,
} from "diff";

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
  type: "lines" | "no-space-lines";
  diff: DiffContent[];
};

type DiffBlocksResult = {
  type: "words" | "chars";
  diff: DiffContent[][];
};

type DiffResult = DiffLinesResult | DiffBlocksResult;

interface DiffDataPair {
  oldLines: string[];
  newLines: string[];
}

const diffLines = ({ oldLines, newLines }: DiffDataPair): DiffResult => {
  const maxLength = Math.max(oldLines.length, newLines.length);

  const diffResult: DiffResult = {
    type: "lines",
    diff: [],
  };
  const lineDiff: DiffContent[] = [];

  for (let i = 0; i < maxLength; i++) {
    const oldLine = oldLines[i] || "";
    const newLine = newLines[i] || "";

    const diff = diffLinesLib(oldLine, newLine);
    diff.forEach((part) => {
      const diffType = mapDiffPartToType(part);
      const content = part.value || "";

      lineDiff.push({
        content,
        diffType,
      });
    });
  }
  diffResult.diff = lineDiff;
  return diffResult;
};

const diffNoSpaceLines = ({ oldLines, newLines }: DiffDataPair): DiffResult => {
  const maxLength = Math.max(oldLines.length, newLines.length);

  const diffResult: DiffResult = {
    type: "no-space-lines",
    diff: [],
  };

  const lineDiff: DiffContent[] = [];
  for (let i = 0; i < maxLength; i++) {
    const oldLine = oldLines[i] || "";
    const newLine = newLines[i] || "";

    const diff = diffLinesLib(oldLine.trim(), newLine.trim());
    diff.forEach((part) => {
      const diffType = mapDiffPartToType(part);
      const content = part.value || "";

      lineDiff.push({
        content,
        diffType,
      });
    });
  }
  diffResult.diff = lineDiff;
  return diffResult;
};

const diffWords = ({ oldLines, newLines }: DiffDataPair): DiffResult => {
  const maxLength = Math.max(oldLines.length, newLines.length);

  const diffResult: DiffResult = {
    type: "words",
    diff: [],
  };
  const wordDiff: DiffContent[][] = [];
  for (let i = 0; i < maxLength; i++) {
    const oldLine = oldLines[i] || "";
    const newLine = newLines[i] || "";

    const diff = diffWordsLib(oldLine, newLine);
    const lineDiff: DiffContent[] = [];
    diff.forEach((part) => {
      const diffType = mapDiffPartToType(part);
      const content = part.value || "";

      lineDiff.push({
        content,
        diffType,
      });
    });
    wordDiff.push(lineDiff);
  }
  diffResult.diff = wordDiff;
  return diffResult;
};

const diffChars = ({ oldLines, newLines }: DiffDataPair): DiffResult => {
  const maxLength = Math.max(oldLines.length, newLines.length);

  const diffResult: DiffResult = {
    type: "chars",
    diff: [],
  };
  const charDiff: DiffContent[][] = [];
  for (let i = 0; i < maxLength; i++) {
    const oldLine = oldLines[i] || "";
    const newLine = newLines[i] || "";

    const diff = diffCharsLib(oldLine, newLine);
    const lineDiff: DiffContent[] = [];
    diff.forEach((part) => {
      const diffType = mapDiffPartToType(part);
      const content = part.value || "";

      lineDiff.push({
        content,
        diffType,
      });
    });
    charDiff.push(lineDiff);
  }
  diffResult.diff = charDiff;
  return diffResult;
};

export const diff = {
  lines: diffLines,
  noSpaceLines: diffNoSpaceLines,
  words: diffWords,
  chars: diffChars,
};

export type { DiffResult, DiffContent };
