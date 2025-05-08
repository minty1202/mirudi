import { useState } from "react";
import type { ExtractedDiffBlockPair } from "@/components/ui";
import { SelectedDiffValueContext } from "./SelectedDiffValueContext";

export const SelectedDiffValueProvider = ({
  children,
}: {
  children: React.ReactNode;
}) => {
  const [value, onHover] = useState<ExtractedDiffBlockPair>({
    left: { fileName: "", data: [] },
    right: { fileName: "", data: [] },
  });

  return (
    <SelectedDiffValueContext.Provider value={{ value, onHover }}>
      {children}
    </SelectedDiffValueContext.Provider>
  );
};
