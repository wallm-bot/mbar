use cxx_qt::Threading;
use std::pin::Pin;

#[cxx_qt::bridge]
mod qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    unsafe extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(f64, temperature)]
        #[qproperty(QString, description)]
        #[qproperty(QString, city)]
        #[qproperty(QString, emoji)]
        type WeatherStatsBackend = super::WeatherStatsBackendRust;

        #[qinvokable]
        fn update_weather(self: Pin<&mut WeatherStatsBackend>);
    }

    impl cxx_qt::Threading for WeatherStatsBackend {}
}

#[derive(Default)]
pub struct WeatherStatsBackendRust {
    temperature: f64,
    description: cxx_qt_lib::QString,
    city: cxx_qt_lib::QString,
    emoji: cxx_qt_lib::QString,
}

impl qobject::WeatherStatsBackend {
    pub fn update_weather(self: Pin<&mut Self>) {
        let qt_thread = self.qt_thread();

        std::thread::spawn(move || {
            let url = "https://api.open-meteo.com/v1/forecast?latitude=14.5763&longitude=121.0392&current_weather=true";

            let json: serde_json::Value = match ureq::get(url).call() {
                Ok(resp) => match resp.into_json() {
                    Ok(j) => j,
                    Err(_) => return,
                },
                Err(_) => return,
            };

            let temp = json["current_weather"]["temperature"]
                .as_f64()
                .unwrap_or(0.0);

            let code = json["current_weather"]["weathercode"].as_i64().unwrap_or(0);

            let (desc, emoji) = match code {
                0 => ("Clear", "☀️"),
                1 | 2 | 3 => ("Cloudy", "☁️"),
                45 | 48 => ("Fog", "🌫️"),
                51..=82 => ("Rain", "🌧️"),
                85 | 86 => ("Snow", "❄️"),
                _ => ("Unknown", "❓"),
            };

            let city = "Mandaluyong";

            // Use the Qt thread to safely update properties
            let _ = qt_thread.queue(move |mut qobject| {
                qobject.as_mut().set_temperature(temp);
                qobject
                    .as_mut()
                    .set_description(cxx_qt_lib::QString::from(desc));
                qobject.as_mut().set_city(cxx_qt_lib::QString::from(city));
                qobject.as_mut().set_emoji(cxx_qt_lib::QString::from(emoji));
            });
        });
    }
}

