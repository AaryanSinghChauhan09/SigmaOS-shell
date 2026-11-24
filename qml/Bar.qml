// Bar — top panel with modules: workspaces, clock, tray, battery, network.
pragma ComponentBehavior: Bound
import QtQuick
import Quickshell
import Quickshell.Wayland
import qs.Commons
import qs.Widgets

PanelWindow {
    id: bar

    required property var config
    readonly property int barHeight: config.height || 32
    readonly property string position: config.position || "top"

    anchors {
        top: position === "top"
        bottom: position === "bottom"
        left: true
        right: true
    }
    implicitHeight: barHeight
    color: Theme.background

    Row {
        anchors.fill: parent
        anchors.leftMargin: 8
        anchors.rightMargin: 8
        spacing: 12

        Workspaces {
            height: barHeight
            visible: config.modules.indexOf("workspaces") >= 0
        }

        Item { width: 1; height: 1 } // spacer (expands via Layout below)

        Row {
            spacing: 12
            BatteryStatus {
                height: barHeight
                visible: config.modules.indexOf("battery") >= 0
            }
            NetworkStatus {
                height: barHeight
                visible: config.modules.indexOf("network") >= 0
            }
            SystemTray {
                height: barHeight
                visible: config.modules.indexOf("system-tray") >= 0
            }
            Clock {
                height: barHeight
                visible: config.modules.indexOf("clock") >= 0
            }
        }
    }
}
