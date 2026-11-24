// Dock — pinned applications launcher bar.
pragma ComponentBehavior: Bound
import QtQuick
import Quickshell
import Quickshell.Wayland
import qs.Commons
import qs.Widgets

PanelWindow {
    id: dock

    required property var config
    readonly property int iconSize: config.iconSize || 48
    readonly property string position: config.position || "bottom"

    anchors {
        bottom: position === "bottom"
        top: position === "top"
        left: true
        right: true
    }
    implicitHeight: iconSize + 16
    color: Theme.background

    Row {
        anchors.centerIn: parent
        spacing: 8

        Repeater {
            model: config.pinned || []
            delegate: DockIcon {
                required property string modelData
                iconSize: dock.iconSize
                appId: modelData
                onClicked: Quickshell.execDetached(["gtk-launch", modelData])
            }
        }
    }
}
