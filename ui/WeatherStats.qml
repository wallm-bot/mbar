import QtQuick
import QtQuick.Controls
import StatusBar 1.0

Item {
    id: root
    implicitWidth: contentRow.width
    implicitHeight: parent ? parent.height : 40

    WeatherStatsBackend {
        id: weather
    }

    Timer {
        interval: 1800000 // 30 minutes in milliseconds
        running: true
        repeat: true
        triggeredOnStart: true
        onTriggered: weather.update_weather()
    }

    Row {
        id: contentRow
        anchors.verticalCenter: parent.verticalCenter
        spacing: 10

        Text {
            text: weather.emoji
            font.pixelSize: 14
            anchors.verticalCenter: parent.verticalCenter
        }

        Column {
            anchors.verticalCenter: parent.verticalCenter
            spacing: 0

            Text {
                text: weather.city
                color: "#aaaaaa"
                font.pixelSize: 9
                font.bold: true
                font.family: "Inter, Roboto, sans-serif"
                font.letterSpacing: 0.3
            }

            Text {
                text: weather.temperature.toFixed(1) + "°C"
                color: "#ffffff"
                font.pixelSize: 12
                font.bold: true
                font.family: "Inter, Roboto, sans-serif"
            }
        }
    }
}