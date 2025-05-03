import useSWR from "swr";
import { z } from "zod";
import { fetcher } from "@/lib";
import camelcaseKeys from "camelcase-keys";
import { diffTypeSchema } from "@/types";

const lineDataSchema = z.object({
  lineno: z.number(),
  content: z.string(),
});

const diffSchema = z.object({
  old: lineDataSchema.nullable(),
  new: lineDataSchema.nullable(),
  diff_type: diffTypeSchema,
});

const diffsFetchSchema = z.record(z.string(), z.array(diffSchema));

type DiffFetchData = z.infer<typeof diffsFetchSchema>;

const diffsSchema = z.record(
  z.string(),
  z.array(z.object({
    old: lineDataSchema.nullable(),
    new: lineDataSchema.nullable(),
    diffType: diffTypeSchema,
  }))
)

export type DiffData = z.infer<typeof diffsSchema>;

export const useDiffData = () => {
  const { data, error, isLoading } = useSWR("/api/diffs", async (url) => {
    const raw = await fetcher<DiffFetchData>(url, diffsFetchSchema);
    const camelized = Object.fromEntries(
      Object.entries(raw).map(([filename, entries]) => [
        filename,
        entries.map((entry) => camelcaseKeys(entry, { deep: true })),
      ])
    );
    return diffsSchema.parse(camelized);
  });

  return {
    data: data as DiffData,
    error,
    isLoading,
  };
};
