import { ReactElement, ReactNode } from "react";

export interface ViewerBoxProps {
  fileName: string;
  children: ReactNode;
}

export function ViewerBox({
  fileName,
  children,
}: ViewerBoxProps): ReactElement {
  return (
    <>
      <div className="border border-gray-300 rounded-md bg-white">
        <div className="text-md font-bold text-gray-700 bg-gray-50 border-b border-gray-300 p-2">
          {fileName}
        </div>
        {children}
      </div>
    </>
  );
}
