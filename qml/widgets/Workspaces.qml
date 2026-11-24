// Workspaces — Hyprland workspace switcher via socket2.
pragma ComponentBehavior: Bound
import QtQuick
import Quickshell
import Quickshell.Wayland
import qs.Commons
import qs.Widgets

Item {
    id: workspaces
    required property int height
    implicitWidth: row.implicitWidth + 16
    implicitHeight: height

    Row {
        id: row
        anchors.verticalCenter: parent.verticalCenter
        spacing: 4

        Repeater {
            model: Hyprland.workspaces
            delegate: Rectangle {
                required property var modelData
                width: 28
                height: 22
                radius: 4
                color: modelData.active ? Theme.accent : Theme.surface
                border.color: Theme.muted
                border.width: modelData.active ? 0 : 1

                Text {
                    anchors.centerIn: parent
                    text: modelData.id
                    color: modelData.active ? "#1a1b26" : Theme.foreground
                    font.pixelSize: 11
                }

                MouseArea {
                    anchors.fill: parent
                    onClicked: Hyprland.dispatch(`workspace ${modelData.id}`)
                }
            }
        }
    }
}
