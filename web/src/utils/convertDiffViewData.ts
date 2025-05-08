import { DiffViewData } from "@/components/ui";
import { DiffData } from "@/hooks";
import { extractExtension } from "@/utils";

interface ConvertedDiffRowArgs {
  diffData: DiffData;
  fileName: string;
}

export const convertDiffViewData = ({
  fileName,
  diffData,
}: ConvertedDiffRowArgs): DiffViewData => {
  const lang = extractExtension(fileName) || "plaintext";

  const formatSide = (side: (typeof diffData)[number]["old" | "new"]) => {
    if (!side) return null;

    return {
      value: {
        lineNumber: side.lineno,
        content: side.content,
      },
      lang,
      diffType: side.diffType,
    };
  };

  const data = diffData.map((diffRow) => ({
    leftData: formatSide(diffRow.old),
    rightData: formatSide(diffRow.new),
  }));

  return {
    fileName,
    diffData: data,
  };
};
