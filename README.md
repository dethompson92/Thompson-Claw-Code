# Thompson-Claw-Code Monorepo

Welcome to the **Thompson-Claw-Code** mega-repo. This repository consolidates five specialized AI agent and CLI tool projects into a single managed workspace.

## Repository Map

This monorepo consists of the following projects, each maintained within its own subdirectory while sharing a unified git history:

| Project | Description | Source |
|         |             |        |
| [**claw-code**](./claw-code) | Canonical Rust implementation of the `claw` CLI agent harness. | [Original Repo](https://github.com/ultraworkers/claw-code) |
| [**clawhip**](./clawhip) | Integration and extensions for the Claw ecosystem. | [Original Repo](https://github.com/Yeachan-Heo/clawhip) |
| [**oh-my-claudecode**](./oh-my-claudecode) | Enhanced capabilities and agents for Claude Code. | [Original Repo](https://github.com/Yeachan-Heo/oh-my-claudecode) |
| [**oh-my-codex**](./oh-my-codex) | Codex-specific agent tools and missions. | [Original Repo](https://github.com/Yeachan-Heo/oh-my-codex) |
| [**oh-my-openagent**](./oh-my-openagent) | Open agent frameworks and packages. | [Original Repo](https://github.com/code-yeongyu/oh-my-openagent) |

## Quick Start

Most projects are either Rust, Python, or TypeScript based. Refer to the individual `README.md` files within each directory for specific build and usage instructions.

- **Claw CLI**: See [claw-code/USAGE.md](./claw-code/USAGE.md)
- **Agents & Skills**: Explore `oh-my-*` directories for specialized agent configurations.

## Development

This repository uses a monorepo structure. When contributing:
1. Ensure your changes are contained within the appropriate project directory.
2. Run tests locally within the specific project folder.
3. Use the root `.gitignore` for workspace-wide exclusions.

---
*Created and organized for Devon Thompson.*
