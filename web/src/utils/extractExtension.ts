export function extractExtension(filePath: string): string | null {
  const lastDotIndex = filePath.lastIndexOf(".");
  if (lastDotIndex === -1 || lastDotIndex === filePath.length - 1) {
    return null;
  }
  return filePath.slice(lastDotIndex + 1).toLowerCase();
}
