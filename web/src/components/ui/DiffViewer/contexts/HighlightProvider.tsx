import { ReactElement, ReactNode } from "react";
import {
  HighlighterContext,
  useHighlighterProvider,
} from "./HighlighterContext";

interface HighlightProviderProps {
  extensions?: string[];
  children: ReactNode;
}

export function HighlightProvider({
  extensions,
  children,
}: HighlightProviderProps): ReactElement {
  const highlighter = useHighlighterProvider(extensions);

  if (!highlighter) {
    return <div>Loading...</div>;
  }

  return (
    <HighlighterContext.Provider value={{ highlighter }}>
      {children}
    </HighlighterContext.Provider>
  );
}
