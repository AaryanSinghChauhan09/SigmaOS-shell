// Clock — time and date display.
pragma ComponentBehavior: Bound
import QtQuick
import QtQuick.Shapes
import qs.Commons
import qs.Widgets

Text {
    id: clock
    required property int height
    anchors.verticalCenter: parent ? parent.verticalCenter : undefined
    text: Qt.formatTime(new Date(), "HH:mm") + "  " + Qt.formatDate(new Date(), "ddd dd MMM")
    color: Theme.foreground
    font.family: Theme.fontFamily
    font.pixelSize: Theme.fontSize

    Timer {
        interval: 1000
        running: true
        repeat: true
        onTriggered: clock.text =
            Qt.formatTime(new Date(), "HH:mm") + "  " + Qt.formatDate(new Date(), "ddd dd MMM")
    }
}
