# Ollama Cloud Model Routing

Workspace defaults for the five active projects. These choices assume an Ollama Pro account and favor three concurrent cloud lanes: one strong default, one upgrade path, and one cheaper fallback path.

## Project Matrix

| Project | App | Default | Upgrade | Fallback | Visual / Large Context |
| --- | --- | --- | --- | --- | --- |
| `clawhip` | `openclaw` | `minimax-m2.7:cloud` | `glm-5.1:cloud` | `qwen3-coder-next:cloud` | `gemini-3-flash-preview:cloud` |
| `claw-code` | `openclaw` | `glm-5.1:cloud` | `glm-5.1:cloud` | `qwen3-coder-next:cloud` | `gemini-3-flash-preview:cloud` |
| `oh-my-openagent` | `opencode` | `kimi-k2.5:cloud` | `glm-5.1:cloud` | `minimax-m2.7:cloud` | `gemini-3-flash-preview:cloud` |
| `oh-my-codex` | `codex` | `glm-5.1:cloud` | `glm-5.1:cloud` | `minimax-m2.7:cloud` | `gemini-3-flash-preview:cloud` |
| `oh-my-claudecode` | `claude` | `kimi-k2.5:cloud` | `glm-5.1:cloud` | `minimax-m2.7:cloud` | `gemini-3-flash-preview:cloud` |

## Launch

Use the workspace launcher from the repository root:

```bash
scripts/ollama-cloud-launch clawhip
scripts/ollama-cloud-launch claw-code
scripts/ollama-cloud-launch oh-my-openagent
scripts/ollama-cloud-launch oh-my-codex
scripts/ollama-cloud-launch oh-my-claudecode
```

Choose a lane explicitly:

```bash
scripts/ollama-cloud-launch oh-my-openagent upgrade
scripts/ollama-cloud-launch clawhip fallback
scripts/ollama-cloud-launch oh-my-claudecode visual
```

Pass extra arguments to the underlying Ollama app launcher after `--`:

```bash
scripts/ollama-cloud-launch oh-my-codex default -- --help
```

## Notes

- Run `ollama signin` once before using cloud models.
- Pulling a cloud tag is optional but useful for early validation, for example `ollama pull glm-5.1:cloud`.
- `oh-my-openagent` has native fallback configuration in `.opencode/oh-my-openagent.jsonc`.
- `claw-code` has native Claw fallback configuration in `.claw/settings.json` and `rust/.claw/settings.json`.
- `oh-my-codex` uses project-local `CODEX_HOME` via `.omx/setup-scope.json` and `.codex/.omx-config.json`.
- `oh-my-claudecode` uses project-local OMC routing in `.claude/omc.jsonc`; Ollama non-Claude models run safest when Claude Code inherits the parent model selected by `ollama launch`.
