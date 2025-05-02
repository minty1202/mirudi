import { useState, useEffect, ReactElement } from "react";
import { createContext, useContext } from "react";
import { createHighlighter, bundledLanguages } from "shiki";
import type { Highlighter } from "shiki";

export type HighlighterContextType = {
  highlighter: Highlighter | null;
};
export const HighlighterContext = createContext<HighlighterContextType | null>(null);

export const useHighlighter = () => {
  const context = useContext(HighlighterContext);
  if (!context) {
    throw new Error("useHighlighter must be used within a HighlighterProvider");
  }
  return context.highlighter;
};

const useHighlighterProvider = () => {
  const [highlighter, setHighlighter] = useState<Highlighter | null>(null);

  useEffect(() => {
    createHighlighter({
      themes: ["github-light"],
      langs: Object.keys(bundledLanguages),
    }).then(setHighlighter);
  }, []);

  return highlighter;
};

export function HighlightProvider({ children }: { children: React.ReactNode }): ReactElement {
  const highlighter = useHighlighterProvider();

  if (!highlighter) {
    return <div>Loading...</div>;
  }

  return (
    <HighlighterContext.Provider value={{ highlighter }}>
      {children}
    </HighlighterContext.Provider>
  );
};
