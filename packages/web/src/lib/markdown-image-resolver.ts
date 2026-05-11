import { getFileUrl } from "@/lib/api-client"
import { normalizePath } from "@/lib/path-utils"

const PASSTHROUGH_RE = /^(https?:|data:|blob:|file:)/i

export function resolveMarkdownImageSrc(
  rawSrc: string,
  projectPath: string | null,
): string {
  if (!rawSrc) return rawSrc
  if (PASSTHROUGH_RE.test(rawSrc)) return rawSrc

  if (!projectPath) return rawSrc

  const pp = normalizePath(projectPath)
  const isAbsolute =
    rawSrc.startsWith("/") || /^[a-zA-Z]:/.test(rawSrc) || rawSrc.startsWith("\\\\")

  if (isAbsolute) return getFileUrl(rawSrc)

  const cleaned = rawSrc.replace(/^\.\//, "")
  const absolute = `${pp}/wiki/${cleaned}`
  return getFileUrl(absolute)
}
