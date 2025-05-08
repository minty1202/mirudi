import { render, screen } from "@testing-library/react";
import { Button } from "./sample";

describe("Button", () => {
  it("ボタンのテキストが表示されること", () => {
    render(<Button>テストボタン</Button>);

    expect(screen.getByText("テストボタン")).toBeInTheDocument();
  });
});
