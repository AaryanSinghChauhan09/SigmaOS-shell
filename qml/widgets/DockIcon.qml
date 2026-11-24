// DockIcon — pinned app icon with running indicator.
pragma ComponentBehavior: Bound
import QtQuick
import qs.Commons
import qs.Widgets

Rectangle {
    id: icon
    required property string appId
    required property int iconSize
    width: iconSize
    height: iconSize + 6
    radius: 8
    color: "transparent"
    border.color: Qt.rgba(1, 1, 1, 0.08)
    border.width: 1

    Column {
        anchors.centerIn: parent
        spacing: 1

        Rectangle {
            width: iconSize - 12
            height: width
            radius: 6
            color: Theme.surface
            border.color: Theme.muted
            border.width: 1

            Text {
                anchors.centerIn: parent
                text: appId.charAt(0).toUpperCase()
                color: Theme.accent
                font.pixelSize: iconSize / 3
                font.bold: true
            }
        }

        // running indicator dot
        Rectangle {
            width: 4
            height: 4
            radius: 2
            color: Theme.accent
            visible: Dock.runningApps.indexOf(appId) >= 0
            anchors.horizontalCenter: parent.horizontalCenter
        }
    }

    signal clicked()

    MouseArea {
        anchors.fill: parent
        hoverEnabled: true
        onClicked: icon.clicked()
    }
}
