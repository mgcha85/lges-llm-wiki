import { getFileUrl } from "@/lib/api-client"

export function convertFileSrc(path: string): string {
  return getFileUrl(path)
}
