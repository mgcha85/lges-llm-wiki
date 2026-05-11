export interface OpenDialogOptions {
  directory?: boolean
  multiple?: boolean
  title?: string
  filters?: Array<{
    name: string
    extensions: string[]
  }>
}

let fileInputEl: HTMLInputElement | null = null

function getFileInput(): HTMLInputElement {
  if (!fileInputEl) {
    fileInputEl = document.createElement("input")
    fileInputEl.type = "file"
    fileInputEl.style.display = "none"
    document.body.appendChild(fileInputEl)
  }
  return fileInputEl
}

export async function open(options?: OpenDialogOptions): Promise<string | string[] | null> {
  return new Promise((resolve) => {
    const input = getFileInput()

    if (options?.directory) {
      input.setAttribute("webkitdirectory", "")
      input.removeAttribute("multiple")
    } else {
      input.removeAttribute("webkitdirectory")
      if (options?.multiple) {
        input.setAttribute("multiple", "")
      } else {
        input.removeAttribute("multiple")
      }
    }

    if (options?.filters && options.filters.length > 0) {
      const exts = options.filters
        .flatMap((f) => f.extensions)
        .filter((e) => e !== "*")
        .map((e) => `.${e}`)
      input.accept = exts.join(",")
    } else {
      input.accept = ""
    }

    const handleChange = () => {
      input.removeEventListener("change", handleChange)
      input.removeEventListener("cancel", handleCancel)

      if (!input.files || input.files.length === 0) {
        resolve(null)
        return
      }

      const paths = Array.from(input.files).map((f) => f.name)
      
      if (options?.multiple) {
        resolve(paths)
      } else {
        resolve(paths[0] ?? null)
      }

      input.value = ""
    }

    const handleCancel = () => {
      input.removeEventListener("change", handleChange)
      input.removeEventListener("cancel", handleCancel)
      resolve(null)
    }

    input.addEventListener("change", handleChange)
    input.addEventListener("cancel", handleCancel)
    input.click()
  })
}
