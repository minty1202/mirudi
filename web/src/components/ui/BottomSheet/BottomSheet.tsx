import { ReactNode } from "react";
import { tv } from "tailwind-variants";

const buttonHeight = "50px";

const bottomSheet = tv({
  base: "fixed bottom-0 left-0 right-0 z-50 bg-white border-t border-gray-300 shadow-lg transition-all duration-300 ease-in-out",
  variants: {
    open: {
      true: "h-[40vh]",
      false: `h-[${buttonHeight}]`,
    },
  },
  defaultVariants: {
    open: false,
  },
});

const contentArea = tv({
  base: `w-full h-[calc(40vh-${buttonHeight})] overflow-y-auto p-4 transition-opacity duration-300`,
  variants: {
    open: {
      true: "opacity-100",
      false: "opacity-0 pointer-events-none",
    },
  },
  defaultVariants: {
    open: false,
  },
});

export interface BottomSheetProps {
  children?: ReactNode;
  open?: boolean;
  onClose?: () => void;
  onOpen?: () => void;
}

export function BottomSheet({
  children,
  open,
  onClose,
  onOpen,
}: BottomSheetProps) {
  return (
    <div className={bottomSheet({ open })}>
      <button
        onClick={() => (open ? onClose?.() : onOpen?.())}
        className={`w-full h-[${buttonHeight}] bg-blue-400 text-white font-medium hover:bg-blue-500`}
      >
        {open ? "閉じる" : "開く"}
      </button>

      <div className={contentArea({ open })}>{children}</div>
    </div>
  );
}
