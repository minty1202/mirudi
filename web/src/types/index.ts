import { z } from "zod";

export const diffTypeSchema = z.enum(["added", "removed", "equal", "replaced"]);
export type DiffType = z.infer<typeof diffTypeSchema>;
