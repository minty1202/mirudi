import { render, screen, fireEvent } from "@testing-library/react";
import { DiffViewer, DiffViewerProps } from "./DiffViewer";

jest.mock("shiki");

jest.mock("./contexts", () => ({
  useHighlighter: () => ({
    codeToHtml: (code: string) => `<pre>${code}</pre>`,
  }),
}));

const selectionCheckerLeft = jest.fn().mockReturnValue(true);
const selectionCheckerRight = jest.fn().mockReturnValue(false);

const onMouseUp = jest.fn();

jest.mock("./hooks", () => ({
  useHoverSelect: () => ({
    left: {
      selectionChecker: selectionCheckerLeft,
    },
    right: {
      selectionChecker: selectionCheckerRight,
    },
    onMouseUp: onMouseUp,
  }),
}));

function renderDiffViewerWithState(props: Partial<DiffViewerProps>) {
  const defaultProps: DiffViewerProps = {
    data: {
      fileName: "example.rs",
      diffData: [
        {
          leftData: {
            value: {
              lineNumber: 1,
              content: 'import { Button } from "./components/ui";',
            },
            lang: "tsx",
            diffType: "equal",
          },
          rightData: {
            value: {
              lineNumber: 1,
              content: 'import { Button } from "./components/ui";',
            },
            lang: "tsx",
            diffType: "equal",
          },
        }
      ],
    },
    value: {
      left: {
        fileName: "",
        data: [],
      },
      right: {
        fileName: "",
        data: [],
      },
    },
    onHover: () => {},
  };
  const mergedProps = { ...defaultProps, ...props };

  return render(
    <DiffViewer
      {...mergedProps}
    />
  );
};


describe("DiffViewer", () => {
  it("DiffViewer が表示されること", () => {
    renderDiffViewerWithState({
    });
    expect(screen.getByText("example.rs")).toBeInTheDocument();
  });

  it("DiffViewer の内容が表示されること", () => {
    const { getAllByText } = renderDiffViewerWithState({});
    expect(getAllByText('import { Button } from "./components/ui";')).toHaveLength(2);
  });

  it("Data が空の場合、DiffViewer が表示されないこと", () => {
    const { queryByText } = renderDiffViewerWithState({
      data: {
        fileName: "",
        diffData: [],
      },
    });
    expect(queryByText("example.rs")).not.toBeInTheDocument();
    expect(queryByText('import { Button } from "./components/ui";')).not.toBeInTheDocument();
  });

  it("leftData がなく、rightData だけがある行は EmptyCell として表示されること", () => {
    const { getByText, container } = renderDiffViewerWithState({
      data: {
        fileName: "example.rs",
        diffData: [
          {
            leftData: undefined,
            rightData: {
              value: {
                lineNumber: 2,
                content:
                  'import { HighlighterContext, useHighlighterProvider } from "./contexts/highlighter";',
              },
              lang: "tsx",
              diffType: "added",
            },
          },
        ],
      },
    });
    expect(getByText("2")).toBeInTheDocument();
    expect(getByText('import { HighlighterContext, useHighlighterProvider } from "./contexts/highlighter";')).toBeInTheDocument();

    const emptyCells = container.querySelectorAll("td.bg-gray-100");
    expect(emptyCells.length).toBe(2);
  });

  it("rightData がなく、leftData だけがある行は EmptyCell として表示されること", () => {
    const { getByText, container } = renderDiffViewerWithState({
      data: {
        fileName: "example.rs",
        diffData: [
          {
            leftData: {
              value: {
                lineNumber: 2,
                content:
                  'import { HighlighterContext, useHighlighterProvider } from "./contexts/highlighter";',
              },
              lang: "tsx",
              diffType: "removed",
            },
            rightData: undefined,
          },
        ],
      },
    });
    expect(getByText("2")).toBeInTheDocument();
    expect(getByText('import { HighlighterContext, useHighlighterProvider } from "./contexts/highlighter";')).toBeInTheDocument();

    const emptyCells = container.querySelectorAll("td.bg-gray-100");
    expect(emptyCells.length).toBe(2);
  });

  it("selectionCheckerLeft に lineNumber が渡される", () => {
    renderDiffViewerWithState({
      data: {
        fileName: "example.rs",
        diffData: [
          {
            leftData: {
              value: {
                lineNumber: 42,
                content: "const x = 1;",
              },
              lang: "ts",
              diffType: "equal",
            },
            rightData: null,
          },
        ],
      },
    });
  
    expect(selectionCheckerLeft).toHaveBeenCalledWith(42);
    expect(selectionCheckerRight).not.toHaveBeenCalled();
  });

  it("selectionCheckerRight に lineNumber が渡される", () => {
    renderDiffViewerWithState({
      data: {
        fileName: "example.rs",
        diffData: [
          {
            leftData: null,
            rightData: {
              value: {
                lineNumber: 42,
                content: "const x = 1;",
              },
              lang: "ts",
              diffType: "equal",
            },
          },
        ],
      },
    });
  
    expect(selectionCheckerRight).toHaveBeenCalledWith(42);
    expect(selectionCheckerLeft).not.toHaveBeenCalled();
  });

  it("マウスアップ時に onMouseUp が呼ばれること", () => {
    const { container } = renderDiffViewerWithState({
      data: {
        fileName: "example.rs",
        diffData: [
          {
            leftData: {
              value: {
                lineNumber: 1,
                content: "const x = 1;",
              },
              lang: "ts",
              diffType: "equal",
            },
            rightData: null,
          },
        ],
      },
    });
  
    const tds = container.querySelectorAll("td");
    fireEvent.mouseUp(tds[0]);
  
    expect(onMouseUp).toHaveBeenCalled();
  });
});
