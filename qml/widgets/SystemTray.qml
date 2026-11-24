// SystemTray — tray icon host via StatusNotifierWatcher.
pragma ComponentBehavior: Bound
import QtQuick
import Quickshell
import Quickshell.Wayland
import qs.Commons
import qs.Widgets

Row {
    id: tray
    required property int height
    spacing: 6
    anchors.verticalCenter: parent ? parent.verticalCenter : undefined

    Repeater {
        model: SystemTray.items
        delegate: Item {
            required property var modelData
            width: 20
            height: 20
            // Tray icon rendering is handled by Quickshell's
            // StatusNotifierItem; this host reserves layout space.
        }
    }
}
