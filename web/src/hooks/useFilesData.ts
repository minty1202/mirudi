import useSWR from "swr";
import { z } from "zod";
import { fetcher } from "@/lib";

const filesDataSchema = z.array(z.string());
type FilesData = z.infer<typeof filesDataSchema>;

export const useFilesData = () => {
  const { data, error, isLoading } = useSWR("/api/files", async (url) => {
    const raw = await fetcher<FilesData>(url, filesDataSchema);
    return filesDataSchema.parse(raw);
  });

  return {
    data,
    error,
    isLoading,
  };
};
