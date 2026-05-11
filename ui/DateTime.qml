import QtQuick
import StatusBar 1.0

Item {
    id: root
    implicitWidth: contentRow.width
    implicitHeight: parent ? parent.height : 40

    signal clicked()

    DateTimeBackend {
        id: backend
    }

    Timer {
        interval: 1000
        running: true
        repeat: true
        triggeredOnStart: true
        onTriggered: backend.update_time()
    }

    Row {
        id: contentRow
        anchors.verticalCenter: parent.verticalCenter
        spacing: 12

        // Day + Date stacked
        // Day + Date stacked
        Item {
            id: dateContainer
            width: Math.max(dateColumn.width, 60)
            height: parent.height
            anchors.verticalCenter: parent.verticalCenter

            Column {
                id: dateColumn
                anchors.centerIn: parent
                spacing: 0

                Text {
                    color: "#aaaaaa"
                    text: backend.day
                    font.pixelSize: 10
                    font.family: "Inter, Roboto, sans-serif"
                    font.letterSpacing: 0.3
                }
                Text {
                    color: "#ffffff"
                    text: backend.date
                    font.pixelSize: 12
                    font.bold: true
                    font.family: "Inter, Roboto, sans-serif"
                }
            }

            TapHandler {
                onTapped: root.clicked()
            }
        }

        // Time
        Text {
            anchors.verticalCenter: parent.verticalCenter
            color: "#ffffff"
            text: backend.time
            font.pixelSize: 16
            font.bold: true
            font.family: "Inter, Roboto, sans-serif"
        }

        // Timezone
        Text {
            anchors.verticalCenter: parent.verticalCenter
            color: "#aaaaaa"
            text: backend.timezone
            font.pixelSize: 9
            font.letterSpacing: 0.3
            font.family: "Inter, Roboto, sans-serif"
        }
    }
}
