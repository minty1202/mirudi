import { DiffViewData, ExtractedDiffBlockPair } from "..";
import { DiffLine } from "../DiffCell";

export const convertDiffBlock = (data: DiffViewData): ExtractedDiffBlockPair => {
  const { fileName, diffData } = data

  const leftData: DiffLine[] = [];
  const rightData: DiffLine[] = [];

  diffData.forEach((row) => {
    if (row.leftData) {
      leftData.push(row.leftData.value);
    }
    if (row.rightData) {
      rightData.push(row.rightData.value);
    }
  });

  return {
    left: {
      fileName,
      data: leftData,
    },
    right: {
      fileName,
      data: rightData,
    },
  };
}

export const extractSelectedLines = (range: { start: number, end: number }, data: DiffLine[]) => {
  const { start, end } = range;

  const [s, e] = [start, end].sort((a, b) => a - b);
  return data.filter(line => line.lineNumber >= s && line.lineNumber <= e);
}

