# mbar

`mbar` is a lightweight, real-time system monitoring dashboard built with Rust and Qt. It provides a beautiful and informative interface to keep track of your system's health, the weather, your schedule, and the time.

## 🚀 Features

- **System Monitoring:** Real-time CPU, RAM, Disk, and NVIDIA GPU (via NVML) utilization.
- **Weather Integration:** Current weather updates (temperature, description, emoji) fetched from Open-Meteo.
- **Google Calendar:** View your upcoming calendar events directly in the dashboard.
- **Time & Date:** Clear display of time, day, date, and timezone.
- **High Performance:** Powered by Rust and optimized with `mimalloc` for low overhead.
- **Modern UI:** Smooth and responsive interface built with QML and `cxx-qt`.

## 🛠️ Tech Stack

- **Language:** [Rust](https://www.rust-lang.org/)
- **GUI Framework:** [Qt](https://www.qt.io/) via [cxx-qt](https://github.com/KDAB/cxx-qt)
- **System Info:** `sysinfo`, `nvml-wrapper`
- **Networking:** `ureq`
- **Serialization:** `serde`, `serde_json`
- **OAuth2:** `oauth2` (for Google Calendar integration)

## 📦 Installation & Running

### Prerequisites

- [Rust toolchain](https://rustup.rs/) (latest stable)
- [Qt 6](https://www.qt.io/download) (required for `cxx-qt` integration)
- C++ build tools (for `cxx-qt`)

### Setup Google Calendar (Optional)

To use the Google Calendar feature, you need to:
1. Create a Google Cloud Project and enable the Google Calendar API.
2. Configure OAuth2 credentials.
3. Create a `client_secret.json` file in the project root with the following structure:

```json
{
  "installed": {
    "client_id": "YOUR_CLIENT_ID",
    "client_secret": "YOUR_CLIENT_SECRET",
    "auth_uri": "https://accounts.google.com/o/oauth2/auth",
    "token_uri": "https://oauth2.googleapis.com/token"
  }
}
```

### Running the Application

1. **Clone the repository:**
   ```bash
   git clone <repository-url>
   cd mbar
   ```

2. **Perform Google Login (if needed):**
   ```bash
   cargo run -- google-login
   ```

3. **Start the dashboard:**
   ```bash
   cargo run
   ```

## 📁 Project Structure

```text
mbar/
├── src/
│   ├── main.rs              # Application entry point
│   ├── system_stats.rs      # CPU, RAM, Disk, and GPU monitoring
│   ├── weather_stats.rs     # Weather data integration
│   ├── datetime_stats.rs    # Time and date utilities
│   └── google_calendar.rs   # Google Calendar API & OAuth2 integration
├── ui/
│   └── main.qml             # QML User Interface
├── Cargo.toml               # Rust dependencies and configuration
└── README.md                # Project documentation
```
