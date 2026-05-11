use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    CxxQtBuilder::new_qml_module(QmlModule::new("StatusBar").version(1, 0))
        .file("src/datetime_stats.rs")
        .file("src/system_stats.rs")
        .file("src/weather_stats.rs")
        .file("src/google_calendar.rs")
        .build();

}
