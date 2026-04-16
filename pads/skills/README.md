# Solar skills library

Each skill is a self-contained markdown file with a description (used for
embedding retrieval at top-k=5) and a canonical invocation snippet. The
orchestrator consults these instead of re-deriving the shape of each tool
every time.

**Rules:**
- Keep each skill under ~50 lines.
- Start with a one-sentence description that answers "when do I use this?".
- Include the canonical invocation (exact command, exact expected exit code).
- Include one test case agent can run to verify the skill works in the
  current workspace.
- Never describe a tool that doesn't exist in this fork at the time of writing.

Append new skills; update existing ones in a PR that cites the lesson (from
`pads/lessons.jsonl`) that motivated the change.
