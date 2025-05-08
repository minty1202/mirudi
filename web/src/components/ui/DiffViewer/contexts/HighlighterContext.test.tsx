import {
  act,
  render,
  waitFor,
  screen,
  renderHook,
  RenderHookResult,
} from "@testing-library/react";
import { HighlightProvider, useHighlighter } from "./HighlighterContext";
import type { Highlighter } from "shiki";

jest.mock("shiki", () => ({
  createHighlighter: () =>
    Promise.resolve({
      codeToHtml: () => "<pre>mocked</pre>",
    }),
  bundledLanguages: {},
}));

describe("HighlighterContext", () => {
  describe("HighlightProvider", () => {
    it("初期化中は Loading... が表示されること", async () => {
      await act(async () => {
        render(
          <HighlightProvider>
            <div>Loading...</div>
          </HighlightProvider>,
        );
      });

      expect(await screen.findByText("Loading...")).toBeInTheDocument();
    });

    it("初期化後は子コンポーネントが表示されること", async () => {
      await act(async () => {
        render(
          <HighlightProvider>
            <div>Ready!</div>
          </HighlightProvider>,
        );
      });

      await waitFor(() => {
        expect(screen.getByText("Ready!")).toBeInTheDocument();
      });
    });
  });

  describe("useHighlighter", () => {
    it("コンテキストが正しく取得できること", async () => {
      const wrapper = ({ children }: { children: React.ReactNode }) => (
        <HighlightProvider>{children}</HighlightProvider>
      );

      let result: RenderHookResult<unknown, Highlighter | null>["result"];

      await act(async () => {
        result = renderHook(() => useHighlighter(), { wrapper }).result;
      });

      expect(result!.current).toBeTruthy();
    });

    it("HighlighterContext が null の場合にエラーが発生すること", () => {
      expect(() => {
        renderHook(() => useHighlighter(), {
          wrapper: ({ children }) => <div>{children}</div>,
        });
      }).toThrow("useHighlighter must be used within a HighlighterProvider");
    });
  });
});
