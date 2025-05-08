import { ZodSchema } from "zod";

export const fetcher = async <T>(
  url: string,
  schema: ZodSchema<T>,
): Promise<T> => {
  const baseUrl = "http://127.0.0.1:3210";
  const fullUrl = url.startsWith("http") ? url : `${baseUrl}${url}`;

  try {
    const response = await fetch(fullUrl, {
      method: "GET",
      headers: {
        "Content-Type": "application/json",
      },
    });

    if (!response.ok) {
      throw new Error("Network response was not ok");
    }

    const data = await response.json();
    return schema.parse(data);
  } catch (error) {
    console.error("Fetch error:", error);
    throw error;
  }
};
