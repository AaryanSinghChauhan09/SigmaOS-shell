#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShellComponent { Panel, Launcher, NotificationCenter }

#[derive(Debug, Default)]
pub struct DesktopShell {
    active_components: Vec<ShellComponent>,
}

impl DesktopShell {
    pub fn new() -> Self { Self::default() }
    pub fn enable_component(&mut self, c: ShellComponent) { self.active_components.push(c); }
    pub fn is_enabled(&self, c: ShellComponent) -> bool { self.active_components.contains(&c) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_desktop_shell_components() {
        let mut shell = DesktopShell::new();
        shell.enable_component(ShellComponent::Panel);
        assert!(shell.is_enabled(ShellComponent::Panel));
        assert!(!shell.is_enabled(ShellComponent::Launcher));
    }
}
