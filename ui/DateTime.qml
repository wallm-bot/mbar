import QtQuick
import StatusBar 1.0
import ".:"

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

        DateComponent {
            day: backend.day
            date: backend.date
            clickable: true
            onClicked: root.clicked()
        }

        TimeComponent {
            time: backend.time
            timezone: backend.timezone
            onTimezoneToggled: backend.request_timezone_change()
        }
    }
}
