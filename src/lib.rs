// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-shell — Desktop shell process: Wayland panel, app launcher widget,
//! and notification center. Inspired by Quickshell / Waybar / Hyprpanel.
//!
//! Design principles: single responsibility, fail-fast, no unsafe code,
//! deterministic unit-testable logic, policy in configuration.

#![forbid(unsafe_code)]

use std::{fmt, collections::VecDeque};

// ── Error ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShellError {
    ComponentAlreadyEnabled(ShellComponent),
    ComponentNotEnabled(ShellComponent),
    LauncherQueryEmpty,
    NotificationExpired(u32),
    InvalidNotificationTimeout,
}

impl fmt::Display for ShellError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ComponentAlreadyEnabled(c) => write!(f, "{c:?} already enabled"),
            Self::ComponentNotEnabled(c)     => write!(f, "{c:?} not enabled"),
            Self::LauncherQueryEmpty          => write!(f, "launcher query is empty"),
            Self::NotificationExpired(id)     => write!(f, "notification {id} expired/not found"),
            Self::InvalidNotificationTimeout  => write!(f, "notification timeout must be > 0"),
        }
    }
}

impl std::error::Error for ShellError {}

// ── Shell components ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShellComponent {
    Panel,
    Launcher,
    NotificationCenter,
    SystemTray,
    ClockWidget,
}

// ── App entry for launcher ────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppEntry {
    pub name:       String,
    pub exec:       String,
    pub categories: Vec<String>,
    pub icon:       Option<String>,
}

impl AppEntry {
    pub fn new(name: impl Into<String>, exec: impl Into<String>) -> Self {
        Self { name: name.into(), exec: exec.into(), categories: vec![], icon: None }
    }

    pub fn with_category(mut self, cat: impl Into<String>) -> Self {
        self.categories.push(cat.into());
        self
    }

    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }
}

// ── Launcher ──────────────────────────────────────────────────────────────────

/// Fuzzy app launcher (rofi-style). Scores by prefix → substring → category.
#[derive(Debug, Default)]
pub struct AppLauncher {
    apps: Vec<AppEntry>,
}

impl AppLauncher {
    pub fn new() -> Self { Self::default() }

    pub fn register(&mut self, app: AppEntry) {
        self.apps.push(app);
    }

    /// Search apps. Fail-fast on empty query. Returns ranked results.
    pub fn search(&self, query: &str) -> Result<Vec<&AppEntry>, ShellError> {
        if query.trim().is_empty() {
            return Err(ShellError::LauncherQueryEmpty);
        }
        let q = query.to_lowercase();
        let mut results: Vec<(&AppEntry, u8)> = self.apps.iter()
            .filter_map(|app| {
                let name = app.name.to_lowercase();
                if name.starts_with(&q)       { Some((app, 2)) }
                else if name.contains(&q)     { Some((app, 1)) }
                else if app.categories.iter().any(|c| c.to_lowercase().contains(&q)) {
                    Some((app, 0))
                } else { None }
            })
            .collect();
        results.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.name.cmp(&b.0.name)));
        Ok(results.into_iter().map(|(a, _)| a).collect())
    }
}

// ── Notifications ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Notification {
    pub id:      u32,
    pub app:     String,
    pub summary: String,
    pub body:    String,
    /// Timeout in milliseconds; 0 means persistent.
    pub timeout: u32,
}

#[derive(Debug, Default)]
pub struct NotificationCenter {
    next_id:       u32,
    notifications: VecDeque<Notification>,
    history:       Vec<Notification>,
    max_visible:   usize,
}

impl NotificationCenter {
    pub fn new(max_visible: usize) -> Self {
        Self { max_visible: max_visible.max(1), ..Self::default() }
    }

    /// Push a new notification. Returns its assigned ID.
    pub fn push(&mut self, app: impl Into<String>, summary: impl Into<String>,
                body: impl Into<String>, timeout_ms: u32) -> Result<u32, ShellError> {
        if timeout_ms == 0 { return Err(ShellError::InvalidNotificationTimeout); }
        self.next_id += 1;
        let n = Notification {
            id: self.next_id,
            app: app.into(),
            summary: summary.into(),
            body: body.into(),
            timeout: timeout_ms,
        };
        if self.notifications.len() >= self.max_visible {
            if let Some(old) = self.notifications.pop_front() {
                self.history.push(old);
            }
        }
        self.notifications.push_back(n);
        Ok(self.next_id)
    }

    /// Dismiss a notification by ID.
    pub fn dismiss(&mut self, id: u32) -> Result<(), ShellError> {
        let pos = self.notifications.iter().position(|n| n.id == id)
            .ok_or(ShellError::NotificationExpired(id))?;
        let removed = self.notifications.remove(pos).unwrap();
        self.history.push(removed);
        Ok(())
    }

    pub fn active(&self) -> &VecDeque<Notification> { &self.notifications }
    pub fn history(&self) -> &[Notification] { &self.history }
    pub fn count(&self) -> usize { self.notifications.len() }
}

// ── Panel ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PanelPosition { Top, Bottom, Left, Right }

#[derive(Debug)]
pub struct Panel {
    pub position:  PanelPosition,
    pub height_px: u16,
    pub modules:   Vec<String>,
}

impl Panel {
    pub fn new(position: PanelPosition, height_px: u16) -> Self {
        Self { position, height_px, modules: Vec::new() }
    }

    pub fn add_module(&mut self, module: impl Into<String>) {
        self.modules.push(module.into());
    }

    pub fn module_count(&self) -> usize { self.modules.len() }
}

// ── Desktop shell orchestrator ────────────────────────────────────────────────

/// Top-level shell process — manages panel, launcher, and notification center.
#[derive(Debug)]
pub struct DesktopShell {
    active: std::collections::HashSet<ShellComponent>,
    pub panel:  Option<Panel>,
    pub launcher: AppLauncher,
    pub notifications: NotificationCenter,
}

impl DesktopShell {
    pub fn new() -> Self {
        Self {
            active:        std::collections::HashSet::new(),
            panel:         None,
            launcher:      AppLauncher::new(),
            notifications: NotificationCenter::new(5),
        }
    }

    /// Enable a shell component. Fail-fast if already enabled.
    pub fn enable(&mut self, c: ShellComponent) -> Result<(), ShellError> {
        if self.active.contains(&c) {
            return Err(ShellError::ComponentAlreadyEnabled(c));
        }
        self.active.insert(c);
        Ok(())
    }

    /// Disable a shell component.
    pub fn disable(&mut self, c: ShellComponent) -> Result<(), ShellError> {
        if !self.active.remove(&c) {
            return Err(ShellError::ComponentNotEnabled(c));
        }
        Ok(())
    }

    pub fn is_enabled(&self, c: ShellComponent) -> bool { self.active.contains(&c) }

    pub fn set_panel(&mut self, panel: Panel) { self.panel = Some(panel); }
}

impl Default for DesktopShell {
    fn default() -> Self { Self::new() }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enable_disable_component() {
        let mut shell = DesktopShell::new();
        shell.enable(ShellComponent::Panel).unwrap();
        assert!(shell.is_enabled(ShellComponent::Panel));
        shell.disable(ShellComponent::Panel).unwrap();
        assert!(!shell.is_enabled(ShellComponent::Panel));
    }

    #[test]
    fn test_double_enable_fails() {
        let mut shell = DesktopShell::new();
        shell.enable(ShellComponent::Launcher).unwrap();
        assert!(matches!(
            shell.enable(ShellComponent::Launcher),
            Err(ShellError::ComponentAlreadyEnabled(_))
        ));
    }

    #[test]
    fn test_disable_not_enabled_fails() {
        let mut shell = DesktopShell::new();
        assert!(matches!(
            shell.disable(ShellComponent::NotificationCenter),
            Err(ShellError::ComponentNotEnabled(_))
        ));
    }

    #[test]
    fn test_launcher_search_prefix_wins() {
        let mut shell = DesktopShell::new();
        shell.launcher.register(AppEntry::new("Firefox", "firefox"));
        shell.launcher.register(AppEntry::new("Files", "nautilus"));
        let results = shell.launcher.search("fi").unwrap();
        // Both match "fi" — prefix ordering: "Files" then "Firefox" alphabetically
        assert_eq!(results.len(), 2);
        // First result should be a prefix match
        assert!(results[0].name.to_lowercase().starts_with("fi"));
    }

    #[test]
    fn test_launcher_empty_query_fails() {
        let shell = DesktopShell::new();
        assert!(matches!(shell.launcher.search("  "), Err(ShellError::LauncherQueryEmpty)));
    }

    #[test]
    fn test_launcher_category_match() {
        let mut launcher = AppLauncher::new();
        launcher.register(AppEntry::new("GIMP", "gimp").with_category("Graphics"));
        let results = launcher.search("graphics").unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "GIMP");
    }

    #[test]
    fn test_notification_push_dismiss() {
        let mut nc = NotificationCenter::new(3);
        let id = nc.push("app", "Hello", "World", 5000).unwrap();
        assert_eq!(nc.count(), 1);
        nc.dismiss(id).unwrap();
        assert_eq!(nc.count(), 0);
        assert_eq!(nc.history().len(), 1);
    }

    #[test]
    fn test_notification_overflow_to_history() {
        let mut nc = NotificationCenter::new(2);
        nc.push("a", "1", "", 1000).unwrap();
        nc.push("b", "2", "", 1000).unwrap();
        nc.push("c", "3", "", 1000).unwrap(); // oldest pushed to history
        assert_eq!(nc.count(), 2);
        assert_eq!(nc.history().len(), 1);
    }

    #[test]
    fn test_notification_zero_timeout_rejected() {
        let mut nc = NotificationCenter::new(5);
        assert!(matches!(
            nc.push("x", "y", "z", 0),
            Err(ShellError::InvalidNotificationTimeout)
        ));
    }

    #[test]
    fn test_panel_modules() {
        let mut panel = Panel::new(PanelPosition::Top, 32);
        panel.add_module("clock");
        panel.add_module("battery");
        assert_eq!(panel.module_count(), 2);
        assert_eq!(panel.position, PanelPosition::Top);
    }

    #[test]
    fn test_shell_panel_set() {
        let mut shell = DesktopShell::new();
        shell.set_panel(Panel::new(PanelPosition::Bottom, 28));
        assert!(shell.panel.is_some());
        assert_eq!(shell.panel.unwrap().position, PanelPosition::Bottom);
    }
}
