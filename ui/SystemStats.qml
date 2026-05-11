import QtQuick
import StatusBar 1.0

Item {
    id: root

    implicitWidth: mainCol.implicitWidth
    implicitHeight: parent ? parent.height : 40

    SystemStatsBackend {
        id: statsBackend
    }

    // Refresh data every 2s
    Timer {
        interval: 2000
        running: true
        repeat: true
        triggeredOnStart: true
        onTriggered: statsBackend.update_stats()
    }

    // Rotate current stat every 5s
    property int currentIndex: 0
    property bool showDetail: false

    Timer {
        interval: 5000
        running: true
        repeat: true
        onTriggered: root.currentIndex = (root.currentIndex + 1) % 4
    }

    Column {
        id: mainCol
        anchors.verticalCenter: parent.verticalCenter
        spacing: 2
        width: 40 // Consistent width with progress bar

        Text {
            text: ["CPU", "RAM", "DSK", "GPU"][root.currentIndex]
            color: ["#34d399", "#60a5fa", "#fb923c", "#a78bfa"][root.currentIndex]
            font.pixelSize: 8
            font.bold: true
            font.family: "Inter, Roboto, sans-serif"
            font.letterSpacing: 0.5
            anchors.horizontalCenter: parent.horizontalCenter
        }

        Rectangle {
            width: 36; height: 3; radius: 2; color: "#333333"
            anchors.horizontalCenter: parent.horizontalCenter
            Rectangle {
                width: parent.width * (getUsage() / 100.0)
                height: parent.height; radius: parent.radius
                color: ["#34d399", "#60a5fa", "#fb923c", "#a78bfa"][root.currentIndex]
                opacity: 0.9
                Behavior on width { NumberAnimation { duration: 400; easing.type: Easing.OutCubic } }
            }
        }

        Text {
            text: getValueText()
            color: "#ffffff"
            font.pixelSize: 9
            font.bold: true
            font.family: "Inter, Roboto, sans-serif"
            anchors.horizontalCenter: parent.horizontalCenter
        }
    }

    function getUsage() {
        switch(currentIndex) {
            case 0: return statsBackend.cpu_usage
            case 1: return statsBackend.ram_usage
            case 2: return statsBackend.disk_usage
            case 3: return statsBackend.gpu_usage
            default: return 0
        }
    }

    function getValueText() {
        if (!showDetail || currentIndex === 0) {
            return Math.round(getUsage()) + "%"
        }
        
        switch(currentIndex) {
            case 1: return statsBackend.ram_gb.toFixed(1) + "G"
            case 2: return statsBackend.disk_gb < 1024.0 ? statsBackend.disk_gb.toFixed(1) + "G" : (statsBackend.disk_gb / 1024.0).toFixed(1) + "T"
            case 3: return statsBackend.gpu_vram_gb.toFixed(1) + "G"
            default: return ""
        }
    }

    MouseArea {
        anchors.fill: parent
        cursorShape: Qt.PointingHandCursor
        onClicked: root.showDetail = !root.showDetail
    }
}
