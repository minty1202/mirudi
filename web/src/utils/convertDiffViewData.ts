import { DiffViewData } from "@/components/ui";
import { DiffData } from "@/hooks";
import { extractExtension } from "@/utils";

export const convertDiffViewData = (diffData: DiffData): DiffViewData[] => {
  return Object.entries(diffData).map(([fileName, diffRows]) => {
    const fileExtension = extractExtension(fileName);
    const diffData = diffRows.map((diffRow) => {
      const leftData = diffRow.old
        ? {
            value: {
              lineNumber: diffRow.old.lineno,
              content: diffRow.old.content,
            },
            lang: fileExtension || "plaintext",
            diffType: diffRow.diffType,
          }
        : null;

      const rightData = diffRow.new
        ? {
            value: {
              lineNumber: diffRow.new.lineno,
              content: diffRow.new.content,
            },
            lang: fileExtension || "plaintext",
            diffType: diffRow.diffType,
          }
        : null;

      return { leftData, rightData };
    });

    return { fileName, diffData };
  });
};
