import { useState } from "react"
import {
  Dialog, DialogContent, DialogHeader, DialogTitle, DialogFooter,
} from "@/components/ui/dialog"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Textarea } from "@/components/ui/textarea"

interface CreateMemoDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  onCreated: (title: string, content: string) => void
}

export function CreateMemoDialog({ open, onOpenChange, onCreated }: CreateMemoDialogProps) {
  const [title, setTitle] = useState("")
  const [content, setContent] = useState("")
  const [error, setError] = useState("")

  function handleCreate() {
    if (!title.trim()) {
      setError("Title is required")
      return
    }
    if (!content.trim()) {
      setError("Content is required")
      return
    }
    onCreated(title.trim(), content.trim())
    setTitle("")
    setContent("")
    setError("")
    onOpenChange(false)
  }

  function handleClose() {
    setTitle("")
    setContent("")
    setError("")
    onOpenChange(false)
  }

  return (
    <Dialog open={open} onOpenChange={handleClose}>
      <DialogContent className="max-w-2xl">
        <DialogHeader>
          <DialogTitle>New Memo</DialogTitle>
        </DialogHeader>
        <div className="flex flex-col gap-4 py-4">
          <div className="flex flex-col gap-2">
            <Label htmlFor="memo-title">Title</Label>
            <Input
              id="memo-title"
              value={title}
              onChange={(e) => setTitle(e.target.value)}
              placeholder="My research notes"
            />
          </div>
          <div className="flex flex-col gap-2">
            <Label htmlFor="memo-content">Content</Label>
            <Textarea
              id="memo-content"
              value={content}
              onChange={(e: React.ChangeEvent<HTMLTextAreaElement>) => setContent(e.target.value)}
              placeholder="Write your memo here..."
              className="min-h-[200px] resize-y"
            />
          </div>
          {error && <p className="text-sm text-destructive">{error}</p>}
        </div>
        <DialogFooter>
          <Button variant="outline" onClick={handleClose}>Cancel</Button>
          <Button onClick={handleCreate}>Create</Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
