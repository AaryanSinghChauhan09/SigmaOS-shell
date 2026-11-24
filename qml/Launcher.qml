// Launcher — fuzzy application launcher overlay.
pragma ComponentBehavior: Bound
import QtQuick
import Quickshell
import Quickshell.Io
import qs.Commons
import qs.Widgets

FloatingWindow {
    id: launcher

    required property var config
    visible: false
    color: "transparent"

    anchors { centerIn: parent }
    implicitWidth: 560
    implicitHeight: 420

    Rectangle {
        anchors.fill: parent
        radius: 12
        color: Theme.surface
        border.color: Theme.accent
        border.width: 1

        Column {
            anchors.fill: parent
            anchors.margins: 12
            spacing: 8

            TextInput {
                id: query
                anchors.left: parent.left
                anchors.right: parent.right
                placeholderText: "Type to search applications..."
                color: Theme.foreground
                font.pixelSize: 16
                onAccepted: launchSelected()
                Keys.onEscapePressed: launcher.close()
            }

            ListView {
                id: results
                anchors.left: parent.left
                anchors.right: parent.right
                anchors.bottom: parent.bottom
                clip: true
                model: AppIndex.results(query.text, config.maxResults || 50)

                delegate: Rectangle {
                    required property var modelData
                    width: results.width
                    height: 40
                    color: index === results.currentIndex ? Theme.accent : "transparent"
                    radius: 6

                    Text {
                        anchors.verticalCenter: parent.verticalCenter
                        anchors.left: parent.left
                        anchors.leftMargin: 8
                        text: modelData.name
                        color: Theme.foreground
                    }

                    MouseArea {
                        anchors.fill: parent
                        onClicked: {
                            results.currentIndex = index
                            launchSelected()
                        }
                    }
                }
            }
        }
    }

    function launchSelected() {
        const item = results.model[results.currentIndex];
        if (item) {
            Quickshell.execDetached(item.exec);
            launcher.close();
        }
    }

    function open() {
        query.text = ""
        query.forceActiveFocus()
        visible = true
    }

    function close() {
        visible = false
    }
}
