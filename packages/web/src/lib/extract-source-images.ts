/**
 * Image extraction orchestration for the ingest pipeline.
 *
 * Dispatch layer over server API endpoints for PDF/Office image extraction.
 * Decides which endpoint to call based on file extension, computes the
 * destination directory (`wiki/media/<source-slug>/`), and gives back
 * a small markdown snippet ready to paste into the LLM's source context.
 */
import { getFileName, normalizePath } from "@/lib/path-utils"
import * as api from "@/lib/api-client"

export type SavedImage = api.SavedImage

const SUPPORTED_PDF_EXTS = ["pdf"] as const
const SUPPORTED_OFFICE_EXTS = ["pptx", "docx", "ppt", "doc"] as const

export async function extractAndSaveSourceImages(
  projectPath: string,
  sourcePath: string,
): Promise<SavedImage[]> {
  const pp = normalizePath(projectPath)
  const sp = normalizePath(sourcePath)
  const fileName = getFileName(sp)
  const ext = fileName.split(".").pop()?.toLowerCase() ?? ""

  const isPdf = (SUPPORTED_PDF_EXTS as readonly string[]).includes(ext)
  const isOffice = (SUPPORTED_OFFICE_EXTS as readonly string[]).includes(ext)
  if (!isPdf && !isOffice) return []

  const slug = fileName.replace(/\.[^.]+$/, "")
  const destDir = `${pp}/wiki/media/${slug}`
  const relTo = `${pp}/wiki`

  try {
    if (isPdf) {
      return await api.extractPdfImages(sp, destDir, relTo)
    } else {
      return await api.extractOfficeImages(sp, destDir, relTo)
    }
  } catch (err) {
    console.warn(
      `[ingest:images] extraction failed for "${fileName}":`,
      err instanceof Error ? err.message : err,
    )
    return []
  }
}

export function buildImageMarkdownSection(
  images: SavedImage[],
  captionsBySha?: Map<string, string>,
): string {
  if (images.length === 0) return ""

  const lines: string[] = ["", "", "## Embedded Images", ""]
  const byPage = new Map<string, SavedImage[]>()
  for (const img of images) {
    const key = img.page == null ? "Document" : `Page ${img.page}`
    const bucket = byPage.get(key)
    if (bucket) bucket.push(img)
    else byPage.set(key, [img])
  }

  const ordered = [...byPage.keys()].sort((a, b) => {
    if (a === "Document") return 1
    if (b === "Document") return -1
    const numA = parseInt(a.replace(/\D/g, ""), 10) || 0
    const numB = parseInt(b.replace(/\D/g, ""), 10) || 0
    return numA - numB
  })

  const sanitize = (s: string): string =>
    s.replace(/[\r\n]+/g, " ").replace(/]/g, ")").trim()

  for (const key of ordered) {
    lines.push(`### ${key}`, "")
    for (const img of byPage.get(key) ?? []) {
      const caption = captionsBySha?.get(img.sha256)
      const alt = caption ? sanitize(caption) : ""
      lines.push(`![${alt}](${img.relPath})`)
    }
    lines.push("")
  }

  return lines.join("\n")
}
