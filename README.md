# Thompson-Claw-Code Monorepo

Welcome to the **Thompson-Claw-Code** mega-repo. This repository consolidates ten specialized AI agent, CLI tool, plugin, skill, and workflow reference projects into a single managed workspace.

## Repository Map

This monorepo consists of the following projects, each maintained within its own subdirectory while sharing a unified git history:

| Project | Description | Source |
| --- | --- | --- |
| [**claw-code**](./claw-code) | Canonical Rust implementation of the `claw` CLI agent harness. | [Original Repo](https://github.com/ultraworkers/claw-code) |
| [**clawhip**](./clawhip) | Integration and extensions for the Claw ecosystem. | [Original Repo](https://github.com/Yeachan-Heo/clawhip) |
| [**oh-my-claudecode**](./oh-my-claudecode) | Enhanced capabilities and agents for Claude Code. | [Original Repo](https://github.com/Yeachan-Heo/oh-my-claudecode) |
| [**oh-my-codex**](./oh-my-codex) | Codex-specific agent tools and missions. | [Original Repo](https://github.com/Yeachan-Heo/oh-my-codex) |
| [**oh-my-openagent**](./oh-my-openagent) | Open agent frameworks and packages. | [Original Repo](https://github.com/code-yeongyu/oh-my-openagent) |
| [**rowboat**](./rowboat) | Adjacent local-first AI memory/workflow product and reference implementation, kept separate from the core Claw runtime. | [Original Repo](https://github.com/rowboatlabs/rowboat) |
| [**knowledge-work-plugins**](./knowledge-work-plugins) | Apache-licensed Claude Cowork and Claude Code plugin catalog for role-based skills, commands, connectors, and agent patterns. | [Original Repo](https://github.com/anthropics/knowledge-work-plugins) |
| [**cheetahclaws**](./cheetahclaws) | Adjacent multi-provider Python agent reference with memory, MCP, plugins, skills, tasks, bridges, UI/web, and tool-system patterns. | [Original Repo](https://github.com/SafeRL-Lab/cheetahclaws) |
| [**andrej-karpathy-skills**](./andrej-karpathy-skills) | Small Claude Code/Cursor behavior-guideline skill pack focused on assumptions, simplicity, surgical edits, and verification loops. | [Original Repo](https://github.com/multica-ai/andrej-karpathy-skills) |
| [**claude-mem**](./claude-mem) | Adjacent AGPL-licensed persistent memory and context-compression plugin/runtime for Claude Code, Gemini CLI, OpenCode, and OpenClaw patterns. | [Original Repo](https://github.com/thedotmack/claude-mem) |

`rowboat` is included as an adjacent product reference. Keep its runtime data, service dependencies, and app-specific workflows scoped to `rowboat/`; it is not part of the core Claw CLI runtime.

`knowledge-work-plugins` is included as an adjacent plugin and skill reference. Keep domain plugins, connector configuration, and marketplace examples scoped to `knowledge-work-plugins/`.

`cheetahclaws` is included as an adjacent Python agent reference. Keep its multi-provider runtime, bridges, web UI, and optional integrations scoped to `cheetahclaws/`; it is not part of the core Claw CLI runtime. Note that its repository currently carries an Apache-2.0 `LICENSE` while `pyproject.toml` declares MIT, so preserve upstream attribution and verify licensing before redistribution-sensitive work.

`andrej-karpathy-skills` is included as an adjacent behavior-guideline skill reference. Its README declares MIT, but no standalone license file is currently present upstream, so preserve attribution and verify licensing before redistribution-sensitive work.

`claude-mem` is included as an adjacent persistent-memory reference. Keep its AGPL-3.0 code, plugin hooks, worker service, UI, docs, and generated runtime data scoped to `claude-mem/`.

Anthropic's [`claude-code`](https://github.com/anthropics/claude-code) repository was assessed as a highly relevant upstream reference, but it is not vendored here because its `LICENSE.md` is all-rights-reserved and subject to Anthropic's commercial terms. Use it as an external reference unless this repository's distribution model changes.

[`collection-claude-code-source-code`](https://github.com/chauncygu/collection-claude-code-source-code) was assessed but is not vendored here because it explicitly includes leaked and decompiled Claude Code source archives. See [docs/repo-intake/collection-claude-code-source-code.md](./docs/repo-intake/collection-claude-code-source-code.md) for the local assessment note; the safe successor aspect is represented locally by `cheetahclaws/`.

## Quick Start

Most projects are either Rust, Python, or TypeScript based. Refer to the individual `README.md` files within each directory for specific build and usage instructions.

- **Claw CLI**: See [claw-code/USAGE.md](./claw-code/USAGE.md)
- **Agents & Skills**: Explore `oh-my-*` directories for specialized agent configurations.
- **Rowboat Reference**: See [rowboat/README.md](./rowboat/README.md) for the memory/workflow product surface.
- **Plugin Reference**: See [knowledge-work-plugins/README.md](./knowledge-work-plugins/README.md) for file-based Claude plugin patterns.
- **Python Agent Reference**: See [cheetahclaws/README.md](./cheetahclaws/README.md) for multi-provider agent, memory, MCP, plugin, and bridge patterns.
- **Behavior Skill Reference**: See [andrej-karpathy-skills/README.md](./andrej-karpathy-skills/README.md) for compact Claude Code/Cursor behavior guidance.
- **Memory Runtime Reference**: See [claude-mem/README.md](./claude-mem/README.md) for persistent memory, context compression, hooks, worker service, and search patterns.

## Development

This repository uses a monorepo structure. When contributing:
1. Ensure your changes are contained within the appropriate project directory.
2. Run tests locally within the specific project folder.
3. Use the root `.gitignore` for workspace-wide exclusions.

---
*Created and organized for Devon Thompson.*
