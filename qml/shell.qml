// SigmaOS Shell — main Quickshell entry point.
// Loads bar, dock, launcher, and notification components.

import QtQuick
import Quickshell
import Quickshell.Wayland
import qs.Commons
import qs.Bar
import qs.Dock
import qs.Launcher
import qs.Notifications

ShellRoot {
    id: root

    // Config loaded from ~/.config/sigmaos/shell.toml via shell-config
    property var shellConfig: ConfigStore {}

    Bar {
        config: root.shellConfig.bar
        visible: true
    }

    Dock {
        config: root.shellConfig.dock
        visible: true
    }

    Launcher {
        config: root.shellConfig.launcher
    }

    NotificationCenter {
        anchors { left: true; right: true; top: root.barHeight }
    }
}
