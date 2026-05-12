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

pub fn get_datetime_info<T: chrono::TimeZone>(now: chrono::DateTime<T>) -> DateTimeInfo 
where T::Offset: std::fmt::Display {
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
    use chrono::{TimeZone};

    #[test]
    fn test_get_datetime_info_format() {
        // Use a fixed time to ensure predictable results
        let offset = chrono::FixedOffset::east_opt(8 * 3600).unwrap(); // UTC+8
        let dt = offset.with_ymd_and_hms(2026, 5, 12, 12, 0, 0).unwrap();
        
        let info = get_datetime_info(dt);
        
        assert_eq!(info.time, "12:00:00");
        assert_eq!(info.day, "Tuesday");
        assert_eq!(info.date, "May 12");
        assert_eq!(info.timezone, "UTC+8");
    }

    #[test]
    fn test_timezone_negative_offset() {
        let offset = chrono::FixedOffset::west_opt(5 * 3600).unwrap(); // UTC-5
        let dt = offset.with_ymd_and_hms(2026, 5, 12, 12, 0, 0).unwrap();
        
        let info = get_datetime_info(dt);
        assert_eq!(info.timezone, "UTC-5");
    }

    #[test]
    fn test_time_string_length() {
        let now = chrono::Local::now();
        let info = get_datetime_info(now);
        assert_eq!(info.time.len(), 8);
    }
}
