// NotificationCenter — transient notification toasts.
pragma ComponentBehavior: Bound
import QtQuick
import Quickshell
import Quickshell.Wayland
import qs.Commons
import qs.Widgets

Column {
    id: center
    required property int anchors
    spacing: 8

    Repeater {
        model: NotificationServer.notifications
        delegate: Rectangle {
            required property var modelData
            width: 360
            height: notificationCol.implicitHeight + 16
            radius: 8
            color: Theme.surface
            border.color: Theme.accent
            border.width: 1

            Column {
                id: notificationCol
                anchors.fill: parent
                anchors.margins: 8
                spacing: 4

                Text {
                    text: modelData.summary || ""
                    color: Theme.foreground
                    font.bold: true
                    font.pixelSize: 12
                    width: parent.width
                    elide: Text.ElideRight
                }
                Text {
                    text: modelData.body || ""
                    color: Theme.muted
                    font.pixelSize: 11
                    width: parent.width
                    wrapMode: Text.WordWrap
                    maximumLineCount: 3
                    elide: Text.ElideRight
                }
            }

            Timer {
                interval: 5000
                running: true
                onTriggered: modelData.dismiss()
            }
        }
    }
}
