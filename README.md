# SigmaOS Shell

The SigmaOS desktop shell — a Quickshell/QML layer providing the bar,
dock, launcher, and notification center for the Hyprland session.

## Layout

```
qml/
  shell.qml            entry point (ShellRoot)
  Bar.qml              top panel with modules
  Dock.qml             pinned application dock
  Launcher.qml         fuzzy app launcher overlay
  ConfigStore.qml      shell.toml singleton
  Theme.qml            color/font tokens
  AppIndex.qml         .desktop entry scanner
  widgets/             Bar module widgets
  Notifications/       notification center
config/
  shell.toml           default shell configuration
```

## Data flow

1. `sigmaos-shell-config` writes `~/.config/sigmaos/shell.toml`
2. `ConfigStore` watches the file and reloads
3. `Bar`, `Dock`, `Launcher` bind to config properties
4. `AppIndex` scans `.desktop` entries for the launcher
5. Widgets read system state via Quickshell's Wayland bindings

## Interconnectivity

- `sigmaos-shell-config` — config source of truth
- `sigmaos-hyprland` — window manager bindings
- `sigmaos-notification-center` — notification backend
- `sigmaos-live-media` — wallpaper surface behind the shell

## Principles

- Single responsibility per widget (KISS, SRP)
- Config via TOML, not hardcoded (declarative config)
- Loose coupling: widgets expose `required property var config`
- Fails safe: missing config falls back to defaults
