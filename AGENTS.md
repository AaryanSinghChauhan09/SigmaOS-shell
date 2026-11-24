# AGENTS.md — SigmaOS-shell

Machine-readable rules for AI agents working in this repository.

## Build & Test

```sh
# QML syntax check (requires qmllint)
qmllint qml/

# Runtime: quickshell -p qml/
```

## Rules

- Single `main` branch; feature branches only for work needing tests,
  merged after tests pass.
- GPL-3.0-or-later. SigmaOS identity only; no upstream project references.
- Docs live in the GitHub Wiki; keep `docs/` minimal staging content.
- Follow SOLID, DRY, KISS, YAGNI; fail-fast errors; secure defaults.
- Component manifest: `COMPONENT.toml`. Interconnect: `tools-sigmaos.sh`.
- QML widgets must declare `required property var config` for injection.
- No hardcoded paths outside `~/.config/sigmaos/` and `~/.local/state/sigmaos/`.
