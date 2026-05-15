import QtQuick

Item {
    id: root
    property string time: ""
    property string timezone: ""
    property var timezones: ["India", "Netherlands", "Norway"]
    property int currentTzIndex: 0

    signal timezoneToggled()

    implicitWidth: timeText.width + tzText.width + 20
    implicitHeight: parent ? parent.height : 40

    Row {
        anchors.verticalCenter: parent.verticalCenter
        spacing: 12

        Text {
            id: timeText
            color: "#ffffff"
            text: root.time
            font.pixelSize: 16
            font.bold: true
            font.family: "Inter, Roboto, sans-serif"
        }

        Text {
            id: tzText
            color: "#aaaaaa"
            text: root.timezone
            font.pixelSize: 9
            font.letterSpacing: 0.3
            font.family: "Inter, Roboto, sans-serif"
        }

        TapHandler {
            onTapped: {
                root.currentTzIndex = (root.currentTzIndex + 1) % root.timezones.length
                root.timezoneToggled()
            }
        }
    }
}
