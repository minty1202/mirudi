import { ReactNode } from "react";
import { render, screen, fireEvent } from "@testing-library/react";
import { DiffCellProps, EmptyCell } from "./DiffCell";
import { DiffType } from "@/components/ui/DiffViewer/types";
jest.mock("shiki");

function MockTable({ children }: { children: ReactNode }) {
  return (
    <table>
      <tbody>
        <tr>{children}</tr>
      </tbody>
    </table>
  );
}

describe("DiffCell", () => {
  describe("基本的な描画", () => {
    describe("正常系", () => {
      let DiffCell: any;
  
      beforeEach(() => {
        jest.resetModules();
        jest.doMock("../contexts", () => ({
          useHighlighter: () => ({
            codeToHtml: (code: string) => `<pre>${code}</pre>`,
          }),
        }));
  
        DiffCell = require("./DiffCell").DiffCell;
      });

      function renderDiffCell(props?: Partial<DiffCellProps>) {
        const defaultProps = {
          value: {
            lineNumber: 1,
            content: "hello DiffCell",
          },
          onMouseDown: () => {},
          onMouseEnter: () => {},
          lang: "rs",
          diffType: "equal" as DiffType,
        };
      
        const mergedProps = { ...defaultProps, ...props };
      
        return render(
          <MockTable>
            <DiffCell {...mergedProps} />
          </MockTable>
        );
      }

      it("DiffCell が表示されること", () => {
        renderDiffCell();
        expect(screen.getByText("1")).toBeInTheDocument();
        expect(screen.getByText("hello DiffCell")).toBeInTheDocument();
      });

      describe("diffType", () => {
        it("added の場合に bg-green と + アイコンが適用されること", () => {
          const { container } = renderDiffCell({
            diffType: "added" as DiffType,
          });

          const tds = container.querySelectorAll("td");
          expect(tds[0].className).toContain("bg-green-300");
          expect(tds[1].className).toContain("bg-green-100");
          expect(tds[1].querySelector("span")?.textContent).toBe("+");
        })

        it("removed の場合に bg-red と - アイコンが適用されること", () => {
          const { container } = renderDiffCell({
            diffType: "removed" as DiffType,
          });
    
          const tds = container.querySelectorAll("td");
          expect(tds[0].className).toContain("bg-red-300");
          expect(tds[1].className).toContain("bg-red-100");
          expect(tds[1].querySelector("span")?.textContent).toBe("-");
        })

        it("replaced の場合に bg-yellow と ~ アイコンが適用されること", () => {
          const { container } = renderDiffCell({
            diffType: "replaced" as DiffType,
          });

          const tds = container.querySelectorAll("td");
    
          expect(tds[0].className).toContain("bg-yellow-300");
          expect(tds[1].className).toContain("bg-yellow-100");
          expect(tds[1].querySelector("span")?.textContent).toBe("~");
        })

        it("equal の場合に bg 関連のクラスとアイコンが適用されないこと", () => {
          const { container } = renderDiffCell({
            diffType: "equal" as DiffType,
          });
          const tds = container.querySelectorAll("td");
          expect(tds[0].className).not.toMatch(/bg-/);
          expect(tds[1].className).not.toMatch(/bg-/);
          expect(tds[1].querySelector("span")?.className).toContain("text-transparent");
        })
      });

      describe("selected", () => {
        it("true の場合に after:bg-blue-400 が適用されること", () => {
          const { container } = renderDiffCell({
            selected: true,
          });
    
          const tds = container.querySelectorAll("td");
          expect(tds[0].className).toContain("after:bg-blue-400");
          expect(tds[1].className).toContain("after:bg-blue-400");
        });

        it("false の場合に bg-blue 関連のクラスが適用されないこと", () => {
          const { container } = renderDiffCell({
            selected: false,
          });
    
          const tds = container.querySelectorAll("td");
          expect(tds[0].className).not.toMatch(/bg-blue-/);
          expect(tds[1].className).not.toMatch(/bg-blue-/);
        });
      });

      describe("codeToHtml", () => {
        it("適用されること", () => {
          const codeToHtml = jest.fn().mockReturnValue("<pre>mocked</pre>");

          jest.resetModules();
          jest.doMock("../contexts", () => ({
            useHighlighter: () => ({
              codeToHtml,
            }),
          }));

          const DiffCell = require("./DiffCell").DiffCell;

          render(
            <MockTable>
                  <DiffCell
                    value={{ lineNumber: 1, content: "hello DiffCell" }}
                    onMouseDown={() => {}}
                    onMouseEnter={() => {}}
                    lang="rs"
                    diffType="added"
                  />
            </MockTable>
          );

          expect(codeToHtml).toHaveBeenCalledWith("hello DiffCell", {
            lang: "rs",
            theme: "github-light",
          });
        });

        it("lang が undefined の場合に 'plaintext' が適用されること", () => {
          const codeToHtml = jest.fn().mockReturnValue("<pre>mocked</pre>");

          jest.resetModules();
          jest.doMock("../contexts", () => ({
            useHighlighter: () => ({
              codeToHtml,
            }),
          }));

          const DiffCell = require("./DiffCell").DiffCell;

          render(
            <MockTable>
              <DiffCell
                value={{ lineNumber: 1, content: "hello DiffCell" }}
                onMouseDown={() => {}}
                onMouseEnter={() => {}}
                lang={undefined}
                diffType="added"
              />
            </MockTable>
          );

          expect(codeToHtml).toHaveBeenCalledWith("hello DiffCell", {
            lang: "plaintext",
            theme: "github-light",
          });
        });

        it("content が undefined の場合に空文字列が適用されること", () => {
          const codeToHtml = jest.fn().mockReturnValue("<pre>mocked</pre>");

          jest.resetModules();
          jest.doMock("../contexts", () => ({
            useHighlighter: () => ({
              codeToHtml,
            }),
          }));

          const DiffCell = require("./DiffCell").DiffCell;

          render(
            <MockTable>
              <DiffCell
                value={{ lineNumber: 1, content: undefined }}
                onMouseDown={() => {}}
                onMouseEnter={() => {}}
                lang="rs"
                diffType="added"
              />
            </MockTable>
          );

          expect(codeToHtml).toHaveBeenCalledWith("", {
            lang: "rs",
            theme: "github-light",
          });
        });
 
      });
    });
  
    describe("異常系", () => {
      let DiffCell: any;
  
      beforeEach(() => {
        jest.resetModules();
        jest.doMock("../contexts", () => ({
          useHighlighter: () => null,
        }));
  
        DiffCell = require("./DiffCell").DiffCell;
      });
  
      it("highlighter が null の場合にエラーが発生すること", () => {
        expect(() => {
          render(
            <MockTable>
              <DiffCell
                value={{
                  lineNumber: 1,
                  content: "hello DiffCell",
                }}
                onMouseDown={() => {}}
                onMouseEnter={() => {}}
                lang="rs"
                diffType="added"
              />
            </MockTable>
          );
        }).toThrow("Highlighter context is not available");
      });
    });
  });

  describe("onMouseDown と onMouseEnter の呼び出し", () => {
    let DiffCell: any;

    beforeEach(() => {
      jest.resetModules();
      jest.doMock("../contexts", () => ({
        useHighlighter: () => ({
          codeToHtml: (code: string) => `<pre>${code}</pre>`,
        }),
      }));

      DiffCell = require("./DiffCell").DiffCell;
    });

    function renderDiffCell(props?: Partial<DiffCellProps>) {
      const defaultProps = {
        value: {
          lineNumber: 1,
          content: "hello DiffCell",
        },
        onMouseDown: () => {},
        onMouseEnter: () => {},
        lang: "rs",
        diffType: "equal" as DiffType,
      };
    
      const mergedProps = { ...defaultProps, ...props };
    
      return render(
        <MockTable>
          <DiffCell {...mergedProps} />
        </MockTable>
      );
    }

    it("onMouseDown と onMouseEnter が呼び出されること", () => {
      const onMouseDown = jest.fn();
      const onMouseEnter = jest.fn();
    
      renderDiffCell({
        onMouseDown,
        onMouseEnter,
      });
    
      const tds = screen.getAllByRole("cell");
    
      fireEvent.mouseDown(tds[1]);
      fireEvent.mouseEnter(tds[1]);
    
      expect(onMouseDown).toHaveBeenCalledWith({
        lineNumber: 1,
        content: "hello DiffCell",
      });
      expect(onMouseEnter).toHaveBeenCalledWith({
        lineNumber: 1,
        content: "hello DiffCell",
      });
    });
  });
});

describe("EmptyCell", () => {
  it("EmptyCell が空白の td を2つ返すこと", () => {
    const { container } = render(
      <table>
        <tbody>
          <tr>
            <EmptyCell />
          </tr>
        </tbody>
      </table>
    );

    const tds = container.querySelectorAll("td");
    expect(tds.length).toBe(2);
    expect(tds[0].className).toContain("bg-gray-100");
    expect(tds[1].className).toContain("bg-gray-100");
  });
});
