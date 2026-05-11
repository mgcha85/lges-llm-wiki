import { useEffect, useState } from "react"
import {
  Dialog, DialogContent, DialogHeader, DialogTitle, DialogFooter,
} from "@/components/ui/dialog"
import { Button } from "@/components/ui/button"
import { listProjects } from "@/commands/fs"
import type { WikiProject } from "@/types/wiki"

interface OpenProjectDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  onSelectProject: (project: WikiProject) => void
}

export function OpenProjectDialog({
  open,
  onOpenChange,
  onSelectProject,
}: OpenProjectDialogProps) {
  const [projects, setProjects] = useState<WikiProject[]>([])
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState("")

  useEffect(() => {
    if (!open) return
    let mounted = true

    async function loadProjects() {
      setLoading(true)
      setError("")
      try {
        const items = await listProjects()
        if (!mounted) return
        setProjects(items)
      } catch (err) {
        if (!mounted) return
        setError(String(err))
      } finally {
        if (mounted) setLoading(false)
      }
    }

    loadProjects()
    return () => {
      mounted = false
    }
  }, [open])

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-lg">
        <DialogHeader>
          <DialogTitle>Open Project (DATA_DIR)</DialogTitle>
        </DialogHeader>

        <div className="max-h-[360px] overflow-y-auto py-2">
          {loading && <p className="text-sm text-muted-foreground">Loading projects...</p>}
          {!loading && error && <p className="text-sm text-destructive">{error}</p>}
          {!loading && !error && projects.length === 0 && (
            <p className="text-sm text-muted-foreground">
              No valid wiki projects found in DATA_DIR.
            </p>
          )}

          {!loading && !error && projects.length > 0 && (
            <div className="rounded-lg border">
              {projects.map((project) => (
                <button
                  key={project.path}
                  onClick={() => {
                    onSelectProject(project)
                    onOpenChange(false)
                  }}
                  className="flex w-full items-center justify-between border-b px-4 py-3 text-left transition-colors last:border-b-0 hover:bg-accent"
                >
                  <div className="min-w-0 flex-1">
                    <div className="truncate text-sm font-medium">{project.name}</div>
                    <div className="truncate text-xs text-muted-foreground">{project.path}</div>
                  </div>
                </button>
              ))}
            </div>
          )}
        </div>

        <DialogFooter>
          <Button variant="outline" onClick={() => onOpenChange(false)}>
            Close
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
