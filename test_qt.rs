fn main() {
    let mut app = cxx_qt_lib::QGuiApplication::new();
    if let Some(app_mut) = app.as_mut() {
        app_mut.set_quit_on_last_window_closed(false);
    }
}
