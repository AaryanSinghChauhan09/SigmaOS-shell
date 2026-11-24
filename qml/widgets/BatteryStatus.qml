// BatteryStatus — battery percentage and icon.
pragma ComponentBehavior: Bound
import QtQuick
import qs.Commons
import qs.Widgets

Row {
    id: battery
    required property int height
    spacing: 4
    anchors.verticalCenter: parent ? parent.verticalCenter : undefined

    Text {
        text: Battery.percent !== undefined ? `${Math.round(Battery.percent)}%` : "--"
        color: Battery.percent < 20 ? "#f7768e" : Theme.foreground
        font.family: Theme.fontFamily
        font.pixelSize: Theme.fontSize
        anchors.verticalCenter: parent.verticalCenter
    }
}
