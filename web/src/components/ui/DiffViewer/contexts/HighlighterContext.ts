import { useState, useEffect } from "react";
import { createContext, useContext } from "react";
import { createHighlighter, bundledLanguages } from "shiki";
import type { Highlighter } from "shiki";

export type HighlighterContextType = {
  highlighter: Highlighter | null;
};
export const HighlighterContext = createContext<HighlighterContextType | null>(
  null,
);

export const useHighlighter = () => {
  const context = useContext(HighlighterContext);
  if (!context) {
    throw new Error("useHighlighter must be used within a HighlighterProvider");
  }
  return context.highlighter;
};

export const useHighlighterProvider = (extensions: string[] = []) => {
  const [highlighter, setHighlighter] = useState<Highlighter | null>(null);

  useEffect(() => {
    const validLangs = extensions
      .filter((ext) => ext in bundledLanguages)
      .concat("plaintext");

    createHighlighter({
      themes: ["github-light"],
      langs: validLangs.length > 1 ? validLangs : Object.keys(bundledLanguages),
    }).then(setHighlighter);
  }, [extensions]);

  return highlighter;
};
