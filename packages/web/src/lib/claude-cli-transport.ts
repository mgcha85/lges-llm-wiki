/**
 * Claude Code CLI subprocess transport.
 *
 * NOTE: This transport is NOT available in the web version. The Claude CLI
 * requires native subprocess spawning which isn't possible from a browser.
 * Users should use the standard Anthropic API provider instead.
 *
 * This module exports stub implementations that throw descriptive errors
 * guiding users to switch providers.
 */

import type { LlmConfig } from "@/stores/wiki-store"
import type { ChatMessage, RequestOverrides } from "./llm-providers"
import type { StreamCallbacks } from "./llm-client"

export function createClaudeCodeStreamParser() {
  let sawDelta = false
  let emittedFromAssistant = ""

  return function parseLine(rawLine: string): string | null {
    const line = rawLine.trim()
    if (!line) return null

    let evt: unknown
    try {
      evt = JSON.parse(line)
    } catch {
      return null
    }

    if (!evt || typeof evt !== "object") return null
    const obj = evt as Record<string, unknown>
    const type = obj.type

    if (type === "stream_event") {
      const event = obj.event as Record<string, unknown> | undefined
      if (event?.type === "content_block_delta") {
        const delta = event.delta as Record<string, unknown> | undefined
        if (delta?.type === "text_delta" && typeof delta.text === "string") {
          sawDelta = true
          return delta.text
        }
      }
      return null
    }

    if (type === "assistant") {
      const message = obj.message as Record<string, unknown> | undefined
      const content = message?.content
      if (!Array.isArray(content)) return null
      const text = content
        .map((c) => {
          const cc = c as Record<string, unknown>
          return cc.type === "text" && typeof cc.text === "string" ? cc.text : ""
        })
        .join("")
      if (!text) return null

      if (sawDelta) {
        return null
      }
      if (text.startsWith(emittedFromAssistant)) {
        const novel = text.slice(emittedFromAssistant.length)
        emittedFromAssistant = text
        return novel || null
      }
      emittedFromAssistant = text
      return text
    }

    return null
  }
}

export async function streamClaudeCodeCli(
  _config: LlmConfig,
  _messages: ChatMessage[],
  callbacks: StreamCallbacks,
  _signal?: AbortSignal,
  _overrides?: RequestOverrides,
): Promise<void> {
  callbacks.onError(
    new Error(
      "Claude Code CLI is not available in the web version. " +
      "Please switch to the Anthropic API provider in Settings."
    )
  )
}

export function buildExitError(
  code: number,
  stderr: string,
  unparsedStdout: string = "",
): string {
  if (/unauthenticated|please.*log\s*in|authentication.*failed/i.test(stderr)) {
    return [
      "Claude Code CLI is not authenticated.",
      "Please open a terminal and run `claude` to complete the OAuth login,",
      "then retry. (LLM Wiki only spawns the binary — it can't run the",
      "login flow on your behalf.)",
      stderr ? `\n\n— stderr —\n${stderr}` : "",
    ].join(" ").trim()
  }
  if (stderr) {
    return `claude CLI exited with code ${code}: ${stderr}`
  }
  if (unparsedStdout.trim()) {
    return [
      `claude CLI exited with code ${code} (no stderr).`,
      "Captured stdout output that LLM Wiki couldn't parse — pasting it",
      "here so you can see what the CLI actually emitted:\n",
      unparsedStdout.trim(),
    ].join(" ")
  }
  return [
    `claude CLI exited silently with code ${code}.`,
    "No stdout or stderr was captured — try running `claude -p` in a",
    "terminal with the same prompt to see what's wrong, or switch to",
    "the official Anthropic API in Settings.",
  ].join(" ")
}
