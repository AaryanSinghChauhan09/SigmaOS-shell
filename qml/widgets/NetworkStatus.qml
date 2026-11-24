// NetworkStatus — connectivity indicator (wifi/ethernet).
pragma ComponentBehavior: Bound
import QtQuick
import qs.Commons
import qs.Widgets

Text {
    id: net
    required property int height
    anchors.verticalCenter: parent ? parent.verticalCenter : undefined
    text: "◈"
    color: Network.connected ? Theme.foreground : Theme.muted
    font.pixelSize: Theme.fontSize
}
