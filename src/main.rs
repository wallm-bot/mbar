#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

mod datetime_stats;
mod system_stats;
mod weather_stats;
mod google_calendar;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "google-login" {
        google_calendar::do_login();
        return;
    }

    let mut app = cxx_qt_lib::QGuiApplication::new();

    let mut engine = cxx_qt_lib::QQmlApplicationEngine::new();

    let qml_path = format!("file://{}/ui/main.qml", env!("CARGO_MANIFEST_DIR"));
    let url = cxx_qt_lib::QUrl::from(qml_path.as_str());

    if let Some(engine_mut) = engine.as_mut() {
        engine_mut.load(&url);
    }

    if let Some(app_mut) = app.as_mut() {
        std::process::exit(app_mut.exec());
    }
}
