import type { WikiProject } from "@/types/wiki"
import type { LlmConfig, SearchApiConfig, EmbeddingConfig, MultimodalConfig, OutputLanguage, ProviderConfigs, ProxyConfig } from "@/stores/wiki-store"

const STORE_PREFIX = "llm-wiki-"

function getKey(key: string): string {
  return `${STORE_PREFIX}${key}`
}

function getItem<T>(key: string): T | null {
  try {
    const item = localStorage.getItem(getKey(key))
    return item ? JSON.parse(item) : null
  } catch {
    return null
  }
}

function setItem<T>(key: string, value: T): void {
  localStorage.setItem(getKey(key), JSON.stringify(value))
}

function removeItem(key: string): void {
  localStorage.removeItem(getKey(key))
}

const RECENT_PROJECTS_KEY = "recentProjects"
const LAST_PROJECT_KEY = "lastProject"

export async function getRecentProjects(): Promise<WikiProject[]> {
  return getItem<WikiProject[]>(RECENT_PROJECTS_KEY) ?? []
}

export async function getLastProject(): Promise<WikiProject | null> {
  return getItem<WikiProject>(LAST_PROJECT_KEY)
}

export async function saveLastProject(project: WikiProject): Promise<void> {
  setItem(LAST_PROJECT_KEY, project)
  await addToRecentProjects(project)
}

export async function addToRecentProjects(project: WikiProject): Promise<void> {
  const existing = getItem<WikiProject[]>(RECENT_PROJECTS_KEY) ?? []
  const filtered = existing.filter((p) => p.path !== project.path)
  const updated = [project, ...filtered].slice(0, 10)
  setItem(RECENT_PROJECTS_KEY, updated)
}

const LLM_CONFIG_KEY = "llmConfig"
const PROVIDER_CONFIGS_KEY = "providerConfigs"
const ACTIVE_PRESET_KEY = "activePresetId"

export async function saveLlmConfig(config: LlmConfig): Promise<void> {
  setItem(LLM_CONFIG_KEY, config)
}

export async function loadLlmConfig(): Promise<LlmConfig | null> {
  return getItem<LlmConfig>(LLM_CONFIG_KEY)
}

export async function saveProviderConfigs(configs: ProviderConfigs): Promise<void> {
  setItem(PROVIDER_CONFIGS_KEY, configs)
}

export async function loadProviderConfigs(): Promise<ProviderConfigs | null> {
  return getItem<ProviderConfigs>(PROVIDER_CONFIGS_KEY)
}

export async function saveActivePresetId(id: string | null): Promise<void> {
  setItem(ACTIVE_PRESET_KEY, id)
}

export async function loadActivePresetId(): Promise<string | null> {
  return getItem<string | null>(ACTIVE_PRESET_KEY)
}

const SEARCH_API_KEY = "searchApiConfig"

export async function saveSearchApiConfig(config: SearchApiConfig): Promise<void> {
  setItem(SEARCH_API_KEY, config)
}

export async function loadSearchApiConfig(): Promise<SearchApiConfig | null> {
  return getItem<SearchApiConfig>(SEARCH_API_KEY)
}

const EMBEDDING_KEY = "embeddingConfig"

export async function saveEmbeddingConfig(config: EmbeddingConfig): Promise<void> {
  setItem(EMBEDDING_KEY, config)
}

export async function loadEmbeddingConfig(): Promise<EmbeddingConfig | null> {
  return getItem<EmbeddingConfig>(EMBEDDING_KEY)
}

const MULTIMODAL_KEY = "multimodalConfig"

export async function saveMultimodalConfig(config: MultimodalConfig): Promise<void> {
  setItem(MULTIMODAL_KEY, config)
}

export async function loadMultimodalConfig(): Promise<MultimodalConfig | null> {
  return getItem<MultimodalConfig>(MULTIMODAL_KEY)
}

const PROXY_CONFIG_KEY = "proxyConfig"

export async function saveProxyConfig(config: ProxyConfig): Promise<void> {
  setItem(PROXY_CONFIG_KEY, config)
}

export async function loadProxyConfig(): Promise<ProxyConfig | null> {
  return getItem<ProxyConfig>(PROXY_CONFIG_KEY)
}

export async function removeFromRecentProjects(path: string): Promise<void> {
  const existing = getItem<WikiProject[]>(RECENT_PROJECTS_KEY) ?? []
  const updated = existing.filter((p) => p.path !== path)
  setItem(RECENT_PROJECTS_KEY, updated)

  const last = getItem<WikiProject>(LAST_PROJECT_KEY)
  if (last && last.path === path) {
    removeItem(LAST_PROJECT_KEY)
  }
}

const LANGUAGE_KEY = "language"

export async function saveLanguage(lang: string): Promise<void> {
  setItem(LANGUAGE_KEY, lang)
}

export async function loadLanguage(): Promise<string | null> {
  return getItem<string>(LANGUAGE_KEY)
}

const OUTPUT_LANGUAGE_KEY = "outputLanguage"
const PROJECT_OUTPUT_LANGUAGE_KEY = "projectOutputLanguages"

export async function saveOutputLanguage(lang: OutputLanguage, projectId?: string): Promise<void> {
  if (projectId) {
    const existing = getItem<Record<string, OutputLanguage>>(PROJECT_OUTPUT_LANGUAGE_KEY) ?? {}
    setItem(PROJECT_OUTPUT_LANGUAGE_KEY, { ...existing, [projectId]: lang })
  }
  setItem(OUTPUT_LANGUAGE_KEY, lang)
}

export async function loadOutputLanguage(projectId?: string): Promise<OutputLanguage | null> {
  if (projectId) {
    const projectLanguages = getItem<Record<string, OutputLanguage>>(PROJECT_OUTPUT_LANGUAGE_KEY)
    return projectLanguages?.[projectId] ?? null
  }
  return getItem<OutputLanguage>(OUTPUT_LANGUAGE_KEY)
}

const UPDATE_CHECK_STATE_KEY = "updateCheckState"

export interface PersistedUpdateCheckState {
  enabled: boolean
  lastCheckedAt: number | null
  dismissedVersion: string | null
}

export async function saveUpdateCheckState(state: PersistedUpdateCheckState): Promise<void> {
  setItem(UPDATE_CHECK_STATE_KEY, state)
}

export async function loadUpdateCheckState(): Promise<PersistedUpdateCheckState | null> {
  return getItem<PersistedUpdateCheckState>(UPDATE_CHECK_STATE_KEY)
}
