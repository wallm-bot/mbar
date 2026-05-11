import QtQuick
import QtQuick.Window
import org.kde.layershell 1.0 as LayerShell

Window {
    id: root
    width: 680
    height: 40
    color: "transparent"
    visible: true
    title: "mbar"

    flags: Qt.FramelessWindowHint | Qt.WindowStaysOnTopHint

    // Wayland Layer Shell configuration
    LayerShell.Window.layer: LayerShell.Window.LayerTop
    LayerShell.Window.anchors: LayerShell.Window.AnchorTop
    LayerShell.Window.margins.top: 0
    LayerShell.Window.exclusionZone: 26

    Rectangle {
        anchors.fill: parent
        color: "#000000"
        radius: 16

        // Flush top corners with screen edge
        Rectangle {
            anchors.top: parent.top
            anchors.left: parent.left
            anchors.right: parent.right
            height: parent.radius
            color: "#000000"
        }

        // ── CENTER: DateTime (always centered) ──────────
        DateTime {
            id: dateTime
            anchors.centerIn: parent
            onClicked: calendar.isOpen = !calendar.isOpen
        }

        SystemStats {
            anchors.right: parent.right
            anchors.rightMargin: 12
            anchors.verticalCenter: parent.verticalCenter
        }

        WeatherStats {
            anchors.left: parent.left
            anchors.leftMargin: 12
            anchors.verticalCenter: parent.verticalCenter
        }
    }

    Calendar {
        id: calendar
    }
}
