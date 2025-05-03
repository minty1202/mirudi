import { useState, useEffect, ReactElement, ReactNode } from "react";
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

const useHighlighterProvider = (extensions: string[] = []) => {
  const [highlighter, setHighlighter] = useState<Highlighter | null>(null);

  useEffect(() => {
    const validLangs = extensions
      .filter(ext => ext in bundledLanguages)
      .concat("plaintext");

    createHighlighter({
      themes: ["github-light"],
      langs: validLangs.length > 0 ? validLangs : Object.keys(bundledLanguages),
    }).then(setHighlighter);

  }, [extensions]);

  return highlighter;
};

interface HighlightProviderProps {
  extensions?: string[];
  children: ReactNode;
}

export function HighlightProvider({ extensions, children }: HighlightProviderProps): ReactElement {
  const highlighter = useHighlighterProvider(extensions);

  if (!highlighter) {
    return <div>Loading...</div>;
  }

  return (
    <HighlighterContext.Provider value={{ highlighter }}>
      {children}
    </HighlighterContext.Provider>
  );
};
