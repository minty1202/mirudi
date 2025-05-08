import { convertDiffBlock, extractSelectedLines } from "./helpers";
import { DiffLine } from "../DiffCell";
import type { DiffType } from "@/types";

describe("convertDiffBlock", () => {
  it("fileName と value を持つ DiffLine[] を返すこと", () => {
    const data = {
      fileName: "test.txt",
      diffData: [
        {
          leftData: {
            value: { lineNumber: 1, content: "line 1" },
            lang: "txt",
            diffType: "replaced" as DiffType,
          },
        },
        {
          rightData: {
            value: { lineNumber: 2, content: "line 2" },
            lang: "txt",
            diffType: "replaced" as DiffType,
          },
        },
      ],
    };

    const result = convertDiffBlock(data);

    expect(result).toEqual({
      left: {
        fileName: "test.txt",
        data: [{ lineNumber: 1, content: "line 1" }],
      },
      right: {
        fileName: "test.txt",
        data: [{ lineNumber: 2, content: "line 2" }],
      },
    });
  });
});

describe("extractSelectedLines", () => {
  it("指定した行範囲の DiffLine[] を返すこと", () => {
    const data: DiffLine[] = [
      { lineNumber: 1, content: "line 1" },
      { lineNumber: 2, content: "line 2" },
      { lineNumber: 3, content: "line 3" },
    ];

    const range = { start: 1, end: 2 };
    const result = extractSelectedLines(range, data);

    expect(result).toEqual([
      { lineNumber: 1, content: "line 1" },
      { lineNumber: 2, content: "line 2" },
    ]);
  });
});
