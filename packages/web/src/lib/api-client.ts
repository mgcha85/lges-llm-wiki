const API_BASE = import.meta.env.VITE_API_URL ?? 'http://localhost:3001';

interface ApiResponse<T> {
  ok: boolean;
  data?: T;
  error?: string;
}

async function request<T>(
  endpoint: string,
  options?: RequestInit
): Promise<T> {
  const res = await fetch(`${API_BASE}${endpoint}`, {
    headers: { 'Content-Type': 'application/json' },
    ...options,
  });

  const json: ApiResponse<T> = await res.json();

  if (!json.ok || json.error) {
    throw new Error(json.error ?? 'Request failed');
  }

  return json.data as T;
}

export async function readFile(path: string): Promise<string> {
  const result = await request<{ content: string }>('/api/fs/read', {
    method: 'POST',
    body: JSON.stringify({ path }),
  });
  return result.content;
}

export async function writeFile(path: string, contents: string): Promise<void> {
  await request<void>('/api/fs/write', {
    method: 'POST',
    body: JSON.stringify({ path, contents }),
  });
}

export async function listDirectory(path: string): Promise<FileNode[]> {
  return request<FileNode[]>('/api/fs/list', {
    method: 'POST',
    body: JSON.stringify({ path }),
  });
}

export async function deleteFile(path: string): Promise<void> {
  await request<void>('/api/fs/delete', {
    method: 'DELETE',
    body: JSON.stringify({ path }),
  });
}

export async function createDirectory(path: string): Promise<void> {
  await request<void>('/api/fs/mkdir', {
    method: 'POST',
    body: JSON.stringify({ path }),
  });
}

export interface FileNode {
  name: string;
  path: string;
  isDir: boolean;
  children?: FileNode[];
}

export interface WikiProject {
  id: string;
  name: string;
  path: string;
}

export async function listProjects(): Promise<WikiProject[]> {
  const result = await request<{ projects: WikiProject[] }>('/api/project/list');
  return result.projects;
}

export async function createProject(name: string): Promise<WikiProject> {
  return request<WikiProject>('/api/project/create', {
    method: 'POST',
    body: JSON.stringify({ name }),
  });
}

export async function openProject(name: string): Promise<WikiProject> {
  return request<WikiProject>('/api/project/open', {
    method: 'POST',
    body: JSON.stringify({ name }),
  });
}

export interface UploadedFile {
  name: string;
  path: string;
  size: number;
}

export async function uploadFiles(
  projectName: string,
  files: File[],
  subdir?: string
): Promise<UploadedFile[]> {
  const formData = new FormData();
  for (const file of files) {
    formData.append('files', file);
  }

  const params = new URLSearchParams({ project: projectName });
  if (subdir) {
    params.append('subdir', subdir);
  }

  const res = await fetch(`${API_BASE}/api/upload?${params}`, {
    method: 'POST',
    body: formData,
  });

  const json: ApiResponse<{ files: UploadedFile[] }> = await res.json();

  if (!json.ok || json.error) {
    throw new Error(json.error ?? 'Upload failed');
  }

  return json.data?.files ?? [];
}

export interface ClipRequest {
  title: string;
  url: string;
  content: string;
  projectName?: string;
}

export async function saveClip(clip: ClipRequest): Promise<string> {
  const result = await request<{ path: string }>('/api/clip', {
    method: 'POST',
    body: JSON.stringify(clip),
  });
  return result.path;
}

export interface PendingClip {
  projectPath: string;
  filePath: string;
}

export async function getPendingClips(): Promise<PendingClip[]> {
  return request<PendingClip[]>('/api/clip/pending');
}

export interface LlmProxyRequest {
  url: string;
  method?: string;
  headers?: Record<string, string>;
  body?: string;
}

export async function proxyLlmRequest(req: LlmProxyRequest): Promise<Response> {
  return fetch(`${API_BASE}/api/llm/proxy`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(req),
  });
}

export function getFileUrl(path: string): string {
  const encoded = encodeURIComponent(path);
  return `${API_BASE}/files/${encoded}`;
}

export async function healthCheck(): Promise<boolean> {
  try {
    const res = await fetch(`${API_BASE}/health`);
    return res.ok;
  } catch {
    return false;
  }
}

export async function copyFile(source: string, destination: string): Promise<void> {
  await request<void>('/api/fs/copy', {
    method: 'POST',
    body: JSON.stringify({ source, destination }),
  });
}

export async function fileExists(path: string): Promise<boolean> {
  const result = await request<{ exists: boolean }>('/api/fs/exists', {
    method: 'POST',
    body: JSON.stringify({ path }),
  });
  return result.exists;
}

export async function preprocessFile(path: string): Promise<string> {
  const result = await request<{ content: string }>('/api/fs/preprocess', {
    method: 'POST',
    body: JSON.stringify({ path }),
  });
  return result.content;
}

export async function findRelatedWikiPages(
  projectPath: string,
  sourceName: string
): Promise<string[]> {
  const result = await request<{ pages: string[] }>('/api/fs/find-related', {
    method: 'POST',
    body: JSON.stringify({ projectPath, sourceName }),
  });
  return result.pages;
}

export interface FileBase64 {
  base64: string;
  mimeType: string;
}

export async function readFileAsBase64(path: string): Promise<FileBase64> {
  return request<FileBase64>('/api/fs/read-base64', {
    method: 'POST',
    body: JSON.stringify({ path }),
  });
}

export async function clipServerStatus(): Promise<string> {
  const result = await request<{ status: string }>('/api/clip/status');
  return result.status;
}

export interface SavedImage {
  index: number;
  mimeType: string;
  page: number | null;
  width: number;
  height: number;
  relPath: string;
  absPath: string;
  sha256: string;
}

export async function extractPdfImages(
  sourcePath: string,
  destDir: string,
  relTo: string
): Promise<SavedImage[]> {
  const result = await request<{ images: SavedImage[] }>('/api/extract/pdf-images', {
    method: 'POST',
    body: JSON.stringify({ sourcePath, destDir, relTo }),
  });
  return result.images;
}

export async function extractOfficeImages(
  sourcePath: string,
  destDir: string,
  relTo: string
): Promise<SavedImage[]> {
  const result = await request<{ images: SavedImage[] }>('/api/extract/office-images', {
    method: 'POST',
    body: JSON.stringify({ sourcePath, destDir, relTo }),
  });
  return result.images;
}

export async function copyDirectory(
  source: string,
  destination: string
): Promise<string[]> {
  const result = await request<{ files: string[] }>('/api/fs/copy-directory', {
    method: 'POST',
    body: JSON.stringify({ source, destination }),
  });
  return result.files;
}
