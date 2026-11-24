// ConfigStore — loads and validates shell.toml.
pragma Singleton
import QtQuick
import Quickshell.Io

Singleton {
    id: configStore

    property var bar: ({
        "position": "top",
        "height": 32,
        "autohide": false,
        "modules": ["workspaces", "clock", "system-tray", "battery", "network"]
    })
    property var dock: ({
        "position": "bottom",
        "iconSize": 48,
        "autohide": false,
        "pinned": ["firefox", "terminal", "files"]
    })
    property var launcher: ({
        "provider": "fuzzel",
        "maxResults": 50,
        "fuzzy": true
    })
    property var theme: ({
        "name": "sigmaos-dark",
        "accent": "#7aa2f7",
        "background": "#1a1b26",
        "foreground": "#c0caf5",
        "font": "monospace 11"
    })

    FileView {
        id: configFile
        path: "~/.config/sigmaos/shell.toml"
        watchChanges: true
        onLoaded: configStore.reload()
    }

    function reload() {
        // shell.toml is applied by sigmaos-shell-config;
        // this stub exposes defaults until the config service is wired.
    }
}
