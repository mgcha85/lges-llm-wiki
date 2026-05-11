import type { FileNode, WikiProject } from "@/types/wiki"
import { ensureProjectId, upsertProjectInfo } from "@/lib/project-identity"
import * as api from "@/lib/api-client"

export async function readFile(path: string): Promise<string> {
  return api.readFile(path)
}

export async function writeFile(path: string, contents: string): Promise<void> {
  return api.writeFile(path, contents)
}

export async function listDirectory(path: string): Promise<FileNode[]> {
  const nodes = await api.listDirectory(path)
  return nodes.map(normalizeFileNode)
}

function normalizeFileNode(node: api.FileNode): FileNode {
  return {
    name: node.name,
    path: node.path,
    is_dir: node.isDir,
    children: node.children?.map(normalizeFileNode),
  }
}

export async function copyFile(
  source: string,
  destination: string
): Promise<void> {
  return api.copyFile(source, destination)
}

export async function preprocessFile(path: string): Promise<string> {
  return api.preprocessFile(path)
}

export async function deleteFile(path: string): Promise<void> {
  return api.deleteFile(path)
}

export async function findRelatedWikiPages(
  projectPath: string,
  sourceName: string
): Promise<string[]> {
  return api.findRelatedWikiPages(projectPath, sourceName)
}

export async function createDirectory(path: string): Promise<void> {
  return api.createDirectory(path)
}

export async function fileExists(path: string): Promise<boolean> {
  return api.fileExists(path)
}

export interface FileBase64 {
  base64: string
  mimeType: string
}

export async function readFileAsBase64(path: string): Promise<FileBase64> {
  return api.readFileAsBase64(path)
}

export async function createProject(name: string): Promise<WikiProject> {
  const project = await api.createProject(name)
  const id = await ensureProjectId(project.path)
  await upsertProjectInfo(id, project.path, project.name)
  return { id, name: project.name, path: project.path }
}

export async function openProject(path: string): Promise<WikiProject> {
  const project = await api.openProject(path)
  const id = await ensureProjectId(project.path)
  await upsertProjectInfo(id, project.path, project.name)
  return { id, name: project.name, path: project.path }
}

export async function clipServerStatus(): Promise<string> {
  return api.clipServerStatus()
}

export async function copyDirectory(
  source: string,
  destination: string
): Promise<string[]> {
  return api.copyDirectory(source, destination)
}
