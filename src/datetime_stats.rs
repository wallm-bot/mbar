#[cxx_qt::bridge]
mod qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    unsafe extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(QString, time)]
        #[qproperty(QString, day)]
        #[qproperty(QString, date)]
        #[qproperty(QString, timezone)]
        type DateTimeBackend = super::DateTimeBackendRust;

        #[qinvokable]
        fn update_time(self: Pin<&mut DateTimeBackend>);
    }
}

#[derive(Default)]
pub struct DateTimeBackendRust {
    time: cxx_qt_lib::QString,
    day: cxx_qt_lib::QString,
    date: cxx_qt_lib::QString,
    timezone: cxx_qt_lib::QString,
}

pub struct DateTimeInfo {
    pub time: String,
    pub day: String,
    pub date: String,
    pub timezone: String,
}

pub fn get_datetime_info(now: chrono::DateTime<chrono::Local>) -> DateTimeInfo {
    let current_time = now.format("%H:%M:%S").to_string();
    let current_day = now.format("%A").to_string();
    let current_date = now.format("%b %d").to_string();
    let tz_offset = now.format("%z").to_string();
    // Convert +0800 -> UTC+8
    let tz_str = if tz_offset.len() >= 3 {
        let sign = &tz_offset[0..1];
        let hours: i32 = tz_offset[1..3].parse().unwrap_or(0);
        if sign == "-" {
            format!("UTC-{}", hours)
        } else {
            format!("UTC+{}", hours)
        }
    } else {
        "UTC".to_string()
    };

    DateTimeInfo {
        time: current_time,
        day: current_day,
        date: current_date,
        timezone: tz_str,
    }
}

impl qobject::DateTimeBackend {
    pub fn update_time(mut self: std::pin::Pin<&mut Self>) {
        let info = get_datetime_info(chrono::Local::now());

        self.as_mut().set_time(cxx_qt_lib::QString::from(&info.time));
        self.as_mut().set_day(cxx_qt_lib::QString::from(&info.day));
        self.as_mut().set_date(cxx_qt_lib::QString::from(&info.date));
        self.as_mut().set_timezone(cxx_qt_lib::QString::from(&info.timezone));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Local;

    #[test]
    fn test_get_datetime_info_format() {
        // Create a fixed time: 2026-05-11 21:00:00
        // Since get_datetime_info takes chrono::Local, we should try to mock or at least test the formatting.
        // We can use FixedOffset to simulate different zones if we want deeper tests, 
        // but for now let's verify the format logic with the current Local.
        
        let now = Local::now();
        let info = get_datetime_info(now);
        
        assert!(info.time.contains(':'));
        assert!(!info.day.is_empty());
        assert!(!info.date.is_empty());
        assert!(info.timezone.starts_with("UTC"));
    }

    #[test]
    fn test_timezone_conversion() {
        // This is a bit tricky with Local::now() because it depends on the system environment.
        // However, we can test the internal logic if we were to use a more generic DateTime.
        // For now, let's just ensure it doesn't crash and returns a sane value.
        let info = get_datetime_info(Local::now());
        println!("Timezone string: {}", info.timezone);
        assert!(info.timezone.len() >= 3);
    }
}
