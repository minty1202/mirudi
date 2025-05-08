import useSWR from "swr";
import { z } from "zod";
import { fetcher } from "@/lib";
import camelcaseKeys from "camelcase-keys";
import { diffTypeSchema } from "@/types";

const lineFetchDataSchema = z.object({
  lineno: z.number(),
  content: z.string(),
  diff_type: diffTypeSchema,
});

const lineDataSchema = z.object({
  lineno: z.number(),
  content: z.string(),
  diffType: diffTypeSchema,
});

const diffSchema = z.object({
  old: lineFetchDataSchema.nullable(),
  new: lineFetchDataSchema.nullable(),
});

const diffFetchSchema = z.array(diffSchema);

type DiffFetchData = z.infer<typeof diffFetchSchema>;

const diffsSchema = z.array(
  z.object({
    old: lineDataSchema.nullable(),
    new: lineDataSchema.nullable(),
  }),
);

export type DiffData = z.infer<typeof diffsSchema>;

export const useDiffData = (fileName: string) => {
  const { data, error, isLoading } = useSWR(
    `/api/diff?file=${encodeURIComponent(fileName)}`,
    async (url) => {
      const raw = await fetcher<DiffFetchData>(url, diffFetchSchema);
      const camelized = raw.map((entry) => {
        return camelcaseKeys(entry, { deep: true });
      });
      return diffsSchema.parse(camelized);
    },
  );

  return {
    data: data as DiffData,
    error,
    isLoading,
  };
};
