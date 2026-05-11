import * as api from "@/lib/api-client"

let pluginFetchPromise: Promise<typeof globalThis.fetch> | null = null

export function getHttpFetch(): Promise<typeof globalThis.fetch> {
  if (!pluginFetchPromise) {
    pluginFetchPromise = Promise.resolve(
      createProxiedFetch() as typeof globalThis.fetch
    )
  }
  return pluginFetchPromise
}

function createProxiedFetch(): typeof globalThis.fetch {
  return async (input: RequestInfo | URL, init?: RequestInit): Promise<Response> => {
    const url = typeof input === "string" ? input : input instanceof URL ? input.toString() : input.url
    const method = init?.method ?? "GET"
    const headers = init?.headers
    const body = init?.body

    const headerRecord: Record<string, string> = {}
    if (headers instanceof Headers) {
      headers.forEach((v, k) => { headerRecord[k] = v })
    } else if (headers && typeof headers === "object") {
      Object.assign(headerRecord, headers)
    }

    const needsProxy = shouldUseProxy(url, headerRecord)

    if (needsProxy) {
      return api.proxyLlmRequest({
        url,
        method,
        headers: headerRecord,
        body: typeof body === "string" ? body : undefined,
      })
    }

    return globalThis.fetch(input, init)
  }
}

function shouldUseProxy(url: string, headers: Record<string, string>): boolean {
  const isSameOrigin = url.startsWith("/") || url.startsWith(window.location.origin)
  if (isSameOrigin) return false

  const hasApiKey = Object.keys(headers).some(k =>
    k.toLowerCase().includes("api-key") ||
    k.toLowerCase() === "authorization" ||
    k.toLowerCase() === "x-api-key"
  )

  return hasApiKey
}

export function isFetchNetworkError(err: unknown): boolean {
  if (!(err instanceof Error)) return false
  if (err.name === "AbortError") return false
  if (err.name === "TypeError") return true
  if (err.message === "Load failed") return true
  if (err.message === "Failed to fetch") return true
  if (err.message.includes("network error")) return true
  return false
}
