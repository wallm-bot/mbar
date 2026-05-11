import QtQuick
import QtQuick.Window
import QtQuick.Controls
import QtQuick.Layouts
import org.kde.layershell 1.0 as LayerShell
import StatusBar 1.0

Window {
    // Animate width for a native slide-in from the right
    width: isOpen ? 320 : 0
    height: Screen.height
    color: "transparent"
    flags: Qt.FramelessWindowHint | Qt.WindowStaysOnTopHint | Qt.WindowDoesNotAcceptFocus
    visible: width > 0

    LayerShell.Window.layer: LayerShell.Window.LayerOverlay
    LayerShell.Window.anchors: LayerShell.Window.AnchorRight | LayerShell.Window.AnchorTop | LayerShell.Window.AnchorBottom
    LayerShell.Window.exclusionZone: -1
    LayerShell.Window.keyboardInteractivity: LayerShell.Window.KeyboardInteractivityNone

    property bool isOpen: false
    
    Behavior on width {
        NumberAnimation {
            duration: 350
            easing.type: Easing.OutQuart
        }
    }

    Rectangle {
        id: background
        anchors.fill: parent
        color: "#000000"
        opacity: 1.0
        clip: true // Ensure content doesn't bleed out during width animation
        
        // Sidebar left border
        Rectangle {
            anchors.left: parent.left
            anchors.top: parent.top
            anchors.bottom: parent.bottom
            width: 1
            color: "#333333"
        }

        ColumnLayout {
            anchors.fill: parent
            anchors.margins: 24
            spacing: 24

            // Header
            ColumnLayout {
                spacing: 4
                Layout.fillWidth: true
                
                Text {
                    text: Qt.formatDateTime(new Date(), "yyyy")
                    color: "#60a5fa"
                    font.pixelSize: 14
                    font.bold: true
                    font.family: "Inter, Roboto, sans-serif"
                    font.letterSpacing: 1.0
                }
                
                Text {
                    text: Qt.formatDateTime(new Date(), "MMMM")
                    color: "#ffffff"
                    font.pixelSize: 28
                    font.bold: true
                    font.family: "Inter, Roboto, sans-serif"
                }
            }

            // Divider
            Rectangle {
                Layout.fillWidth: true
                height: 1
                color: "#222222"
            }

            // Calendar section
            ColumnLayout {
                Layout.fillWidth: true
                spacing: 16

                DayOfWeekRow {
                    Layout.fillWidth: true
                    delegate: Text {
                        text: model.shortName
                        color: "#666666"
                        font.pixelSize: 11
                        font.bold: true
                        font.family: "Inter, Roboto, sans-serif"
                        horizontalAlignment: Text.AlignHCenter
                        verticalAlignment: Text.AlignVCenter
                    }
                }

                MonthGrid {
                    id: grid
                    Layout.fillWidth: true
                    month: new Date().getMonth()
                    year: new Date().getFullYear()

                    delegate: Item {
                        implicitWidth: 40
                        implicitHeight: 40

                        Rectangle {
                            anchors.centerIn: parent
                            width: 32
                            height: 32
                            radius: 16
                            color: model.today ? "#2563eb" : "transparent"
                        }

                        Text {
                            anchors.centerIn: parent
                            text: model.day
                            color: model.today ? "#ffffff" : (model.month === grid.month ? "#eeeeee" : "#444444")
                            font.pixelSize: 13
                            font.bold: model.today
                            font.family: "Inter, Roboto, sans-serif"
                        }
                    }
                }
            }

            Item { Layout.fillHeight: true }
            
            // Footer/Close hint
            Text {
                text: "Click date again to close"
                color: "#444444"
                font.pixelSize: 10
                font.family: "Inter, Roboto, sans-serif"
                Layout.alignment: Qt.AlignHCenter
            }
        }
    }

    GoogleCalendarBackend {
        id: googleCalendar
    }

    onIsOpenChanged: {
        if (isOpen) {
            googleCalendar.update_events()
        }
    }

    // Event model parsed from JSON
    property var eventModel: JSON.parse(googleCalendar.events_json)

    // Events list below the calendar
    ColumnLayout {
        anchors.left: background.left
        anchors.right: background.right
        anchors.bottom: background.bottom
        anchors.top: background.top
        anchors.topMargin: 480 // Position below the calendar grid
        anchors.margins: 24
        spacing: 16
        visible: background.x === 0 // Only show when sidebar is open

        Text {
            text: "Today's Schedule"
            color: "#ffffff"
            font.pixelSize: 16
            font.bold: true
            font.family: "Inter, Roboto, sans-serif"
        }

        ListView {
            id: eventListView
            Layout.fillWidth: true
            Layout.fillHeight: true
            model: eventModel
            clip: true
            spacing: 12
            
            delegate: Rectangle {
                width: eventListView.width
                height: 54
                color: "#1a1a1a"
                radius: 8
                
                RowLayout {
                    anchors.fill: parent
                    anchors.margins: 12
                    spacing: 12

                    Rectangle {
                        width: 3
                        Layout.fillHeight: true
                        color: modelData.color || "#4285F4"
                        radius: 2
                    }

                    ColumnLayout {
                        spacing: 2
                        Layout.fillWidth: true
                        
                        Text {
                            text: modelData.summary
                            color: "#ffffff"
                            font.pixelSize: 13
                            font.bold: true
                            font.family: "Inter, Roboto, sans-serif"
                            elide: Text.ElideRight
                            Layout.fillWidth: true
                        }

                        Text {
                            text: modelData.startTime + (modelData.endTime ? " - " + modelData.endTime : "")
                            color: "#888888"
                            font.pixelSize: 11
                            font.family: "Inter, Roboto, sans-serif"
                        }
                    }
                }
            }

            footer: Item {
                width: eventListView.width
                height: 20
            }

            // Loading indicator
            Text {
                anchors.centerIn: parent
                text: "Fetching events..."
                color: "#666666"
                font.pixelSize: 12
                visible: googleCalendar.is_loading && eventModel.length === 0
            }

            // Empty state
            Text {
                anchors.centerIn: parent
                text: "No events for today"
                color: "#666666"
                font.pixelSize: 12
                visible: !googleCalendar.is_loading && eventModel.length === 0
            }
        }
    }
}
