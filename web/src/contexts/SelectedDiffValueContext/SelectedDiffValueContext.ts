import { createContext, useContext } from "react";
import type { DiffViewerProps } from "@/components/ui";

type SelectedDiffValueContextType = Pick<DiffViewerProps, "value" | "onHover">;

export const SelectedDiffValueContext =
  createContext<SelectedDiffValueContextType | null>(null);

export const useSelectedDiffValue = () => {
  const ctx = useContext(SelectedDiffValueContext);
  if (!ctx)
    throw new Error("useHoverValue must be used within HoverValueProvider");
  return ctx;
};
