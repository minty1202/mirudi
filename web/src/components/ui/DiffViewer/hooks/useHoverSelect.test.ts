import { act, renderHook } from "@testing-library/react";
import { useHoverSelect } from "./useHoverSelect";
import { convertDiffBlock } from "../helpers";
import { testDiffData } from "./testDiffData";

import type { DiffViewerProps } from "../DiffViewer";
import type { DiffRowProps } from "../DiffViewer";

const mockProps: DiffViewerProps = {
  data: {
    fileName: "example.rs",
    diffData: testDiffData as DiffRowProps[],
  },
  value: {
    left: { fileName: "example.rs", data: [] },
    right: { fileName: "example.rs", data: [] },
  },
  onHover: jest.fn(),
};

describe("useHoverSelect", () => {
  it("初期化時点では onHover は呼び出されない", () => {
    renderHook(() => useHoverSelect(mockProps));
    expect(mockProps.onHover).not.toHaveBeenCalled();
  });

  describe("handleMouseDown", () => {
    describe("left", () => {
      it("ファイル名と diffLine を onHover で呼び出す", () => {
        const onHoverMock = jest.fn();
        const props = {
          ...mockProps,
          onHover: onHoverMock,
        };
        const { result } = renderHook(() => useHoverSelect(props));
        const handleMouseDown = result.current.left.onMouseDown;
        const diffLine = { lineNumber: 1, content: "test" };
        act(() => {
          handleMouseDown(diffLine);
        });
        expect(onHoverMock).toHaveBeenCalledTimes(1);
        const updaterFn = onHoverMock.mock.calls[0][0];
        const updatedValue = updaterFn({
          left: { fileName: "example.rs", data: [] },
          right: { fileName: "example.rs", data: [] },
        });

        expect(updatedValue).toEqual({
          left: {
            fileName: "example.rs",
            data: [diffLine],
          },
          right: {
            fileName: "example.rs",
            data: [],
          },
        });
      });

      it("すでに行番号が指定されている場合、上書きされる", () => {
        const onHoverMock = jest.fn();
        const props = {
          ...mockProps,
          onHover: onHoverMock,
        };
        const { result } = renderHook(() => useHoverSelect(props));
        const handleMouseDown = result.current.left.onMouseDown;
        const diffLine1 = { lineNumber: 1, content: "test" };
        const diffLine2 = { lineNumber: 2, content: "test" };
        act(() => {
          handleMouseDown(diffLine1);
          handleMouseDown(diffLine2);
        });
        expect(onHoverMock).toHaveBeenCalledTimes(2);
        const updaterFn = onHoverMock.mock.calls[1][0];
        const updatedValue = updaterFn({
          left: { fileName: "example.rs", data: [] },
          right: { fileName: "example.rs", data: [] },
        });

        expect(updatedValue).toEqual({
          left: {
            fileName: "example.rs",
            data: [diffLine2],
          },
          right: {
            fileName: "example.rs",
            data: [],
          },
        });
      });
    });

    describe("right", () => {
      it("ファイル名と diffLine を onHover で呼び出す", () => {
        const onHoverMock = jest.fn();
        const props = {
          ...mockProps,
          onHover: onHoverMock,
        };
        const { result } = renderHook(() => useHoverSelect(props));
        const handleMouseDown = result.current.right.onMouseDown;
        const diffLine = { lineNumber: 1, content: "test" };
        act(() => {
          handleMouseDown(diffLine);
        });
        expect(onHoverMock).toHaveBeenCalledTimes(1);
        const updaterFn = onHoverMock.mock.calls[0][0];
        const updatedValue = updaterFn({
          left: { fileName: "example.rs", data: [] },
          right: { fileName: "example.rs", data: [] },
        });

        expect(updatedValue).toEqual({
          left: {
            fileName: "example.rs",
            data: [],
          },
          right: {
            fileName: "example.rs",
            data: [diffLine],
          },
        });
      });

      it("すでに行番号が指定されている場合、上書きされる", () => {
        const onHoverMock = jest.fn();
        const props = {
          ...mockProps,
          onHover: onHoverMock,
        };
        const { result } = renderHook(() => useHoverSelect(props));
        const handleMouseDown = result.current.right.onMouseDown;
        const diffLine1 = { lineNumber: 1, content: "test" };
        const diffLine2 = { lineNumber: 2, content: "test" };
        act(() => {
          handleMouseDown(diffLine1);
          handleMouseDown(diffLine2);
        });
        expect(onHoverMock).toHaveBeenCalledTimes(2);
        const updaterFn = onHoverMock.mock.calls[1][0];
        const updatedValue = updaterFn({
          left: { fileName: "example.rs", data: [] },
          right: { fileName: "example.rs", data: [] },
        });

        expect(updatedValue).toEqual({
          left: {
            fileName: "example.rs",
            data: [],
          },
          right: {
            fileName: "example.rs",
            data: [diffLine2],
          },
        });
      });
    });
  });

  describe("handleMouseEnter", () => {
    describe("left", () => {
      it("アンカーが null の場合、何もしない", () => {
        const onHoverMock = jest.fn();
        const props = {
          ...mockProps,
          onHover: onHoverMock,
        };
        const { result } = renderHook(() => useHoverSelect(props));
        const handleMouseEnter = result.current.left.onMouseEnter;
        const diffLine = { lineNumber: 1, content: "test" };
        act(() => {
          handleMouseEnter(diffLine);
        });
        expect(onHoverMock).not.toHaveBeenCalled();
      });

      it("現在の行とアンカー行を比較し、ソートする", () => {
        const onHoverMock = jest.fn();
        const props = {
          ...mockProps,
          onHover: onHoverMock,
        };
        const { result } = renderHook(() => useHoverSelect(props));
        const handleMouseDown = result.current.left.onMouseDown;
        const handleMouseEnter = result.current.left.onMouseEnter;

        const diffLine1 = testDiffData[0]?.leftData?.value;
        const diffLine2 = testDiffData[3]?.leftData?.value;

        if (!diffLine1 || !diffLine2) {
          throw new Error("diffLine1 or diffLine2 is null");
        }

        act(() => {
          handleMouseDown(diffLine1);
          handleMouseEnter(diffLine2);
        });

        const diffBlock = convertDiffBlock(mockProps.data);
        const expectedLines = [
          diffBlock.left.data[0],
          diffBlock.left.data[1],
          diffBlock.left.data[2],
        ];
        expect(onHoverMock).toHaveBeenCalledTimes(2);
        const updater1 = onHoverMock.mock.calls[0][0];
        const afterMouseDown = updater1({
          left: { fileName: "example.rs", data: [] },
          right: { fileName: "example.rs", data: [] },
        });

        const updater2 = onHoverMock.mock.calls[1][0];
        const afterMouseEnter = updater2(afterMouseDown);

        expect(afterMouseEnter).toEqual({
          left: {
            fileName: "example.rs",
            data: expectedLines,
          },
          right: {
            fileName: "example.rs",
            data: [],
          },
        });
      });

      it("startLine と endLine が逆になっても期待通りの引数で callBack される", () => {
        const onHoverMock = jest.fn();
        const props = {
          ...mockProps,
          onHover: onHoverMock,
        };
        const { result } = renderHook(() => useHoverSelect(props));
        const handleMouseDown = result.current.left.onMouseDown;
        const handleMouseEnter = result.current.left.onMouseEnter;

        const diffLine1 = testDiffData[0]?.leftData?.value;
        const diffLine2 = testDiffData[3]?.leftData?.value;

        if (!diffLine1 || !diffLine2) {
          throw new Error("diffLine1 or diffLine2 is null");
        }

        act(() => {
          handleMouseDown(diffLine2);
          handleMouseEnter(diffLine1);
        });

        const diffBlock = convertDiffBlock(mockProps.data);
        const expectedLines = [
          diffBlock.left.data[0],
          diffBlock.left.data[1],
          diffBlock.left.data[2],
        ];

        expect(onHoverMock).toHaveBeenCalledTimes(2);

        const updater1 = onHoverMock.mock.calls[0][0];
        const afterMouseDown = updater1({
          left: { fileName: "example.rs", data: [] },
          right: { fileName: "example.rs", data: [] },
        });

        const updater2 = onHoverMock.mock.calls[1][0];
        const afterMouseEnter = updater2(afterMouseDown);

        expect(afterMouseEnter).toEqual({
          left: {
            fileName: "example.rs",
            data: expectedLines,
          },
          right: {
            fileName: "example.rs",
            data: [],
          },
        });
      });
    });

    describe("right", () => {
      it("アンカーが null の場合、何もしない", () => {
        const onHoverMock = jest.fn();
        const props = {
          ...mockProps,
          onHover: onHoverMock,
        };
        const { result } = renderHook(() => useHoverSelect(props));
        const handleMouseEnter = result.current.right.onMouseEnter;
        const diffLine = { lineNumber: 1, content: "test" };
        act(() => {
          handleMouseEnter(diffLine);
        });
        expect(onHoverMock).not.toHaveBeenCalled();
      });

      it("現在の行とアンカー行を比較し、ソートする", () => {
        const onHoverMock = jest.fn();
        const props = {
          ...mockProps,
          onHover: onHoverMock,
        };
        const { result } = renderHook(() => useHoverSelect(props));
        const handleMouseDown = result.current.right.onMouseDown;
        const handleMouseEnter = result.current.right.onMouseEnter;

        const diffLine1 = testDiffData[0]?.rightData?.value;
        const diffLine2 = testDiffData[3]?.rightData?.value;

        if (!diffLine1 || !diffLine2) {
          throw new Error("diffLine1 or diffLine2 is null");
        }

        act(() => {
          handleMouseDown(diffLine1);
          handleMouseEnter(diffLine2);
        });

        const diffBlock = convertDiffBlock(mockProps.data);
        const expectedLines = [
          diffBlock.right.data[0],
          diffBlock.right.data[1],
          diffBlock.right.data[2],
          diffBlock.right.data[3],
        ];

        expect(onHoverMock).toHaveBeenCalledTimes(2);

        const updater1 = onHoverMock.mock.calls[0][0];
        const afterMouseDown = updater1({
          left: { fileName: "example.rs", data: [] },
          right: { fileName: "example.rs", data: [] },
        });

        const updater2 = onHoverMock.mock.calls[1][0];
        const afterMouseEnter = updater2(afterMouseDown);

        expect(afterMouseEnter).toEqual({
          left: {
            fileName: "example.rs",
            data: [],
          },
          right: {
            fileName: "example.rs",
            data: expectedLines,
          },
        });
      });

      it("startLine と endLine が逆になっても期待通りの引数で callBack される", () => {
        const onHoverMock = jest.fn();
        const props = {
          ...mockProps,
          onHover: onHoverMock,
        };
        const { result } = renderHook(() => useHoverSelect(props));
        const handleMouseDown = result.current.right.onMouseDown;
        const handleMouseEnter = result.current.right.onMouseEnter;

        const diffLine1 = testDiffData[0]?.rightData?.value;
        const diffLine2 = testDiffData[3]?.rightData?.value;

        if (!diffLine1 || !diffLine2) {
          throw new Error("diffLine1 or diffLine2 is null");
        }

        act(() => {
          handleMouseDown(diffLine2);
          handleMouseEnter(diffLine1);
        });

        const diffBlock = convertDiffBlock(mockProps.data);
        const expectedLines = [
          diffBlock.right.data[0],
          diffBlock.right.data[1],
          diffBlock.right.data[2],
          diffBlock.right.data[3],
        ];

        expect(onHoverMock).toHaveBeenCalledTimes(2);

        const updater1 = onHoverMock.mock.calls[0][0];
        const afterMouseDown = updater1({
          left: { fileName: "example.rs", data: [] },
          right: { fileName: "example.rs", data: [] },
        });

        const updater2 = onHoverMock.mock.calls[1][0];
        const afterMouseEnter = updater2(afterMouseDown);

        expect(afterMouseEnter).toEqual({
          left: {
            fileName: "example.rs",
            data: [],
          },
          right: {
            fileName: "example.rs",
            data: expectedLines,
          },
        });
      });
    });
  });
  describe("handleMouseUp", () => {
    it("handleMouseUp 実行後は handleMouseEnter が onHover を呼ばない", () => {
      const onHoverMock = jest.fn();
      const { result } = renderHook(() =>
        useHoverSelect({
          ...mockProps,
          onHover: onHoverMock,
        }),
      );

      const diffLine = { lineNumber: 1, content: "test line" };

      act(() => {
        result.current.left.onMouseDown(diffLine);
      });
      expect(onHoverMock).toHaveBeenCalledTimes(1);

      act(() => {
        result.current.onMouseUp();
      });

      act(() => {
        result.current.left.onMouseEnter(diffLine);
      });

      expect(onHoverMock).toHaveBeenCalledTimes(1);
    });
  });

  describe("checkLineSelected", () => {
    describe("left", () => {
      it("ファイル名が異なる場合、false を返す", () => {
        const diffBlock = convertDiffBlock(mockProps.data);
        const diffLine = diffBlock.left.data[0];
        const onHoverMock = jest.fn();
        const props = {
          ...mockProps,
          data: {
            fileName: "example2.rs",
            diffData: testDiffData as DiffRowProps[],
          },
          value: {
            left: { fileName: "example.rs", data: [diffLine] },
            right: { fileName: "example.rs", data: [] },
          },
          onHover: onHoverMock,
        };
        const { result } = renderHook(() => useHoverSelect(props));
        const checkLineSelected = result.current.left.selectionChecker;
        const lineNumber = 1;
        const isSelected = checkLineSelected(lineNumber);
        expect(isSelected).toBe(false);
      });

      it("選択されていない状態（空配列）で、常に false になる", () => {
        const onHoverMock = jest.fn();
        const props = {
          ...mockProps,
          onHover: onHoverMock,
        };
        const { result } = renderHook(() => useHoverSelect(props));
        const checkLineSelected = result.current.left.selectionChecker;
        const lineNumber = 1;
        const isSelected = checkLineSelected(lineNumber);
        expect(isSelected).toBe(false);
      });

      it("期待する行が true になる", () => {
        const onHoverMock = jest.fn();
        const diffBlock = convertDiffBlock(mockProps.data);
        const diffLine = diffBlock.left.data[0];
        const props = {
          ...mockProps,
          onHover: onHoverMock,
          value: {
            left: { fileName: "example.rs", data: [diffLine] },
            right: { fileName: "example.rs", data: [] },
          },
        };

        const { result } = renderHook(() => useHoverSelect(props));

        const checkLineSelected = result.current.left.selectionChecker;

        expect(checkLineSelected(1)).toBe(true);
      });
    });

    describe("right", () => {
      it("ファイル名が異なる場合、false を返す", () => {
        const diffBlock = convertDiffBlock(mockProps.data);
        const diffLine = diffBlock.right.data[0];
        const onHoverMock = jest.fn();
        const props = {
          ...mockProps,
          data: {
            fileName: "example2.rs",
            diffData: testDiffData as DiffRowProps[],
          },
          value: {
            left: { fileName: "example.rs", data: [] },
            right: { fileName: "example.rs", data: [diffLine] },
          },
          onHover: onHoverMock,
        };
        const { result } = renderHook(() => useHoverSelect(props));
        const checkLineSelected = result.current.right.selectionChecker;
        const lineNumber = 1;
        const isSelected = checkLineSelected(lineNumber);
        expect(isSelected).toBe(false);
      });

      it("選択されていない状態（空配列）で、常に false になる", () => {
        const onHoverMock = jest.fn();
        const props = {
          ...mockProps,
          onHover: onHoverMock,
        };
        const { result } = renderHook(() => useHoverSelect(props));
        const checkLineSelected = result.current.right.selectionChecker;
        const lineNumber = 1;
        const isSelected = checkLineSelected(lineNumber);
        expect(isSelected).toBe(false);
      });

      it("期待する行が true になる", () => {
        const onHoverMock = jest.fn();
        const diffBlock = convertDiffBlock(mockProps.data);
        const diffLine = diffBlock.right.data[0];
        const props = {
          ...mockProps,
          onHover: onHoverMock,
          value: {
            left: { fileName: "example.rs", data: [] },
            right: { fileName: "example.rs", data: [diffLine] },
          },
        };

        const { result } = renderHook(() => useHoverSelect(props));
        const checkLineSelected = result.current.right.selectionChecker;

        expect(checkLineSelected(1)).toBe(true);
      });
    });
  });
});
