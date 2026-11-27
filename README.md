<<<<<<< HEAD
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
src/
  lib.rs               Rust component API
  sigmaquickshell.rs   Quickshell IPC bindings (bar/launcher/notifications)
wiki/                  component wiki pages
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
=======
# SigmaOS-shell

Shell component for SigmaOS

- **Category:** Developer Tools
- **Engine:** shell_config
- **Language:** Rust

## Build & Test

```sh
cargo build
cargo test
```

Part of the SigmaOS multi-repository component ecosystem.
>>>>>>> origin/main

## See Also

Related SigmaOS **Desktop** repositories:

- [SigmaOS-shell-config](../SigmaOS-shell-config/README.md) — SigmaOS shell configuration
- [SigmaOS-deskland](../SigmaOS-deskland/README.md) — SigmaOS Deskland desktop environment
- [SigmaOS-hyprland-titlebars](../SigmaOS-hyprland-titlebars/README.md) — SigmaOS Hyprland titlebars
- [SigmaOS-sddm](../SigmaOS-sddm/README.md) — SigmaOS display manager theme & config
- [SigmaOS-ui](../SigmaOS-ui/README.md) — SigmaOS UI toolkit
- [SigmaOS-theme-registry](../SigmaOS-theme-registry/README.md) — SigmaOS theme registry
- [SigmaOS-theme-marketplace](../SigmaOS-theme-marketplace/README.md) — SigmaOS theme marketplace
- [SigmaOS-themes-loops](../SigmaOS-themes-loops/README.md) — SigmaOS animated theme loops
- [SigmaOS-themes-intros](../SigmaOS-themes-intros/README.md) — SigmaOS boot & login theme intros
