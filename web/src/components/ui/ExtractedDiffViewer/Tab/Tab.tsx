import { ReactElement, ReactNode } from "react";
import { tv } from "tailwind-variants";

const tabButton = tv({
  base: 'cursor-pointer py-2 px-4',
  variants: {
    selected: {
      true: "border-b-2 border-blue-500 text-blue-500",
      false: "",
    }
  },
  defaultVariants: {
    selected: false
  }
})

interface TabButtonProps {
  children?: ReactNode;
  selected?: boolean;
  onClick: () => void;
}

export function TabButton({ selected = false, children, onClick }: TabButtonProps): ReactElement {

  return (
    <>
      <button className={tabButton({ selected })} onClick={onClick}>{children}</button>
    </>
  )
}

