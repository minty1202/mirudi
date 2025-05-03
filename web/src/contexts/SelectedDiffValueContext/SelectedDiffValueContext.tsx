import { useState } from "react";
import { createContext, useContext } from "react";
import type { ExtractedDiffBlockPair, DiffViewerProps } from "@/components/ui";

type SelectedDiffValueContextType = Pick<DiffViewerProps, "value" | "onHover">;

export const SelectedDiffValueContext = createContext<SelectedDiffValueContextType | null>(null);

export const useSelectedDiffValue = () => {
  const ctx = useContext(SelectedDiffValueContext);
  if (!ctx) throw new Error("useHoverValue must be used within HoverValueProvider");
  return ctx;
};

export const SelectedDiffValueProvider = ({ children }: { children: React.ReactNode }) => {
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
