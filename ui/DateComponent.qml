import QtQuick

Item {
    id: root
    property string day: ""
    property string date: ""
    property bool clickable: false
    signal clicked()

    implicitWidth: dateColumn.width + 20
    implicitHeight: parent ? parent.height : 40

    Column {
        id: dateColumn
        anchors.centerIn: parent
        spacing: 0

        Text {
            color: "#aaaaaa"
            text: root.day
            font.pixelSize: 10
            font.family: "Inter, Roboto, sans-serif"
            font.letterSpacing: 0.3
        }
        Text {
            color: "#ffffff"
            text: root.date
            font.pixelSize: 12
            font.bold: true
            font.family: "Inter, Roboto, sans-serif"
        }
    }

    TapHandler {
        enabled: root.clickable
        onTapped: root.clicked()
    }
}
