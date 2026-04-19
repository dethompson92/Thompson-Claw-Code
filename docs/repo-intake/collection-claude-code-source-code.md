# collection-claude-code-source-code Intake Note

Source: https://github.com/chauncygu/collection-claude-code-source-code

Assessed branches:
- `main` at `b934603b2800374b315b25061bbeffb40ab6ab26`
- `clawspring` at `4bfc3c386f650fbd1b3f2fc183485f7d06d5c9da`

## Decision

Do not vendor `collection-claude-code-source-code` into this monorepo.

The repository's own README describes major contents as a raw leaked Claude Code source archive and a decompiled/unpacked Claude Code archive reconstructed from `@anthropic-ai/claude-code@2.1.88`. Those materials should not be copied into this repository.

## Safe Local Coverage

The useful clean-room-style direction from that ecosystem is represented locally by:

- [`cheetahclaws`](../../cheetahclaws), imported from https://github.com/SafeRL-Lab/cheetahclaws

Use `cheetahclaws/` for multi-provider Python agent patterns around memory, MCP, plugins, skills, tasks, bridges, UI/web, and tool systems.

## Notes

- `original-source-code/`: do not import; described upstream as raw leaked TypeScript source.
- `claude-code-source-code/`: do not import; described upstream as decompiled/unpacked Anthropic package source.
- `claw-code/`: do not import from this collection; this monorepo already carries the canonical local `claw-code/` workspace.
- `clawspring/`: do not import from this collection snapshot; use the current upstream successor `cheetahclaws/` instead.
