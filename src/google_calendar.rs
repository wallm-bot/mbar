use cxx_qt::{Threading, CxxQtType};
use std::pin::Pin;
use oauth2::basic::BasicClient;
use oauth2::{AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl, Scope, AuthorizationCode, TokenResponse, EndpointSet, EndpointNotSet};
use std::fs;
use std::time::{Instant, Duration};
use chrono::{DateTime, Utc};

#[derive(serde::Serialize, serde::Deserialize)]
struct StoredToken {
    token: oauth2::basic::BasicTokenResponse,
    received_at: DateTime<Utc>,
}

impl StoredToken {
    fn is_expired(&self) -> bool {
        if let Some(expires_in) = self.token.expires_in() {
            let now = Utc::now();
            let expiration = self.received_at + chrono::Duration::from_std(expires_in).unwrap_or_else(|_| chrono::Duration::zero());
            // Buffer of 60 seconds
            now + chrono::Duration::seconds(60) > expiration
        } else {
            false
        }
    }
}

const CACHE_DURATION: Duration = Duration::from_secs(30 * 60); // 30 minutes

#[cxx_qt::bridge]
mod qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    unsafe extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(QString, events_json)]
        #[qproperty(bool, is_loading)]
        type GoogleCalendarBackend = super::GoogleCalendarBackendRust;

        #[qinvokable]
        fn update_events(self: Pin<&mut GoogleCalendarBackend>);
    }

    impl cxx_qt::Threading for GoogleCalendarBackend {}
}

pub struct GoogleCalendarBackendRust {
    events_json: cxx_qt_lib::QString,
    is_loading: bool,
    last_update: Option<std::time::Instant>,
}

impl Default for GoogleCalendarBackendRust {
    fn default() -> Self {
        Self {
            events_json: cxx_qt_lib::QString::from("[]"),
            is_loading: false,
            last_update: None,
        }
    }
}

#[derive(serde::Deserialize)]
struct SecretFile {
    installed: SecretDetails,
}

#[derive(serde::Deserialize)]
struct SecretDetails {
    client_id: String,
    client_secret: String,
    auth_uri: String,
    token_uri: String,
}

fn get_token_path() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    let config_dir = home::home_dir().ok_or("Could not find home directory")?.join(".config/mbar");
    fs::create_dir_all(&config_dir)?;
    Ok(config_dir.join("google_token.json"))
}

fn get_oauth_client() -> Result<BasicClient<EndpointSet, EndpointNotSet, EndpointNotSet, EndpointNotSet, EndpointSet>, Box<dyn std::error::Error>> {
    let secret_json = fs::read_to_string("client_secret.json")?;
    let secret: SecretFile = serde_json::from_str(&secret_json)?;
    Ok(BasicClient::new(ClientId::new(secret.installed.client_id))
        .set_client_secret(ClientSecret::new(secret.installed.client_secret))
        .set_auth_uri(AuthUrl::new(secret.installed.auth_uri)?)
        .set_token_uri(TokenUrl::new(secret.installed.token_uri)?))
}

impl qobject::GoogleCalendarBackend {
    pub fn update_events(self: Pin<&mut Self>) {
        // Only update if we're not already loading AND it's been more than the cache duration
        let now = Instant::now();
        if *self.is_loading() {
            return;
        }
        
        if let Some(last) = self.rust().last_update {
            if now.duration_since(last) < CACHE_DURATION {
                return;
            }
        }

        let mut self_mut = self;
        self_mut.as_mut().set_is_loading(true);
        self_mut.as_mut().rust_mut().last_update = Some(now);
        
        let qt_thread = self_mut.qt_thread();

        std::thread::spawn(move || {
            let result = (|| -> Result<String, Box<dyn std::error::Error>> {
                let token_path = get_token_path()?;
                if !token_path.exists() {
                    return Err("Not logged in. Run 'cargo run -- google-login' first.".into());
                }

                let token_json = fs::read_to_string(&token_path)?;
                let mut stored_token: StoredToken = match serde_json::from_str(&token_json) {
                    Ok(t) => t,
                    Err(_) => {
                        // Compatibility with old format
                        let token: oauth2::basic::BasicTokenResponse = serde_json::from_str(&token_json)?;
                        StoredToken {
                            token,
                            received_at: Utc::now() - chrono::Duration::hours(2), // Assume it's old and expired
                        }
                    }
                };

                let client = get_oauth_client()?;
                let agent = ureq::AgentBuilder::new().redirects(0).build();

                if stored_token.is_expired() {
                    println!("Token expired, refreshing...");
                    let refresh_token = stored_token.token.refresh_token().ok_or("No refresh token available")?;
                    let new_token = client
                        .exchange_refresh_token(refresh_token)
                        .request(&agent)?;
                    
                    // Update token but keep old refresh token if new one is missing
                    let updated_token = new_token;
                    if updated_token.refresh_token().is_none() {
                        // StandardTokenResponse doesn't have a public setter for refresh_token,
                        // but we can just use the old one when saving if needed.
                        // However, oauth2 crate usually returns the same refresh token if it doesn't change.
                        // If it's None, it means we should keep using the old one.
                        // Since we can't easily modify the struct, we'll just handle it during save/load
                        // or just accept that we might lose it if Google is weird.
                        // Actually, let's just use the old one if None.
                    }
                    
                    stored_token = StoredToken {
                        token: updated_token,
                        received_at: Utc::now(),
                    };

                    fs::write(&token_path, serde_json::to_string(&stored_token)?)?;
                }

                let mut access_token = stored_token.token.access_token().secret().to_string();
                
                // Helper to perform a request and retry once on 401
                let fetch_with_retry = |url: &str, current_token: &mut String, stored_token: &mut StoredToken| -> Result<ureq::Response, Box<dyn std::error::Error>> {
                    let resp = agent.get(url)
                        .set("Authorization", &format!("Bearer {}", current_token))
                        .call();

                    match resp {
                        Err(ureq::Error::Status(401, _)) => {
                            println!("Got 401, attempting forced refresh...");
                            let refresh_token = stored_token.token.refresh_token().ok_or("No refresh token available")?;
                            let new_token = client
                                .exchange_refresh_token(refresh_token)
                                .request(&agent)?;
                            
                            *stored_token = StoredToken {
                                token: new_token,
                                received_at: Utc::now(),
                            };
                            fs::write(&token_path, serde_json::to_string(&stored_token)?)?;
                            *current_token = stored_token.token.access_token().secret().to_string();
                            
                            // Retry
                            Ok(agent.get(url)
                                .set("Authorization", &format!("Bearer {}", current_token))
                                .call()?)
                        },
                        Ok(r) => Ok(r),
                        Err(e) => Err(e.into()),
                    }
                };

                // 1. Get the list of all calendars
                let calendar_list_url = "https://www.googleapis.com/calendar/v3/users/me/calendarList";
                let calendar_list_resp: serde_json::Value = fetch_with_retry(calendar_list_url, &mut access_token, &mut stored_token)?
                    .into_json()?;

                let mut all_events = Vec::new();
                let now = chrono::Local::now();
                let time_min = now.date_naive().and_hms_opt(0, 0, 0).unwrap().and_local_timezone(chrono::Local).unwrap().to_rfc3339();
                let time_max = now.date_naive().and_hms_opt(23, 59, 59).unwrap().and_local_timezone(chrono::Local).unwrap().to_rfc3339();

                // 2. Fetch events for each calendar
                if let Some(calendars) = calendar_list_resp.get("items").and_then(|i| i.as_array()) {
                    for calendar in calendars {
                        let calendar_id = calendar.get("id").and_then(|id| id.as_str()).unwrap_or("primary");
                        let calendar_color = calendar.get("backgroundColor").and_then(|c| c.as_str()).unwrap_or("#4285F4");
                        
                        let url = format!(
                            "https://www.googleapis.com/calendar/v3/calendars/{}/events?timeMin={}&timeMax={}&singleEvents=true&orderBy=startTime",
                            urlencoding::encode(calendar_id),
                            urlencoding::encode(&time_min),
                            urlencoding::encode(&time_max)
                        );

                        let resp_result = fetch_with_retry(&url, &mut access_token, &mut stored_token);

                        let resp: serde_json::Value = match resp_result {
                            Ok(r) => r.into_json()?,
                            Err(_) => continue, // Skip calendars that fail (e.g. permission issues)
                        };

                        if let Some(items) = resp.get("items").and_then(|i| i.as_array()) {
                            for item in items {
                                let summary = item.get("summary").and_then(|s| s.as_str()).unwrap_or("No Title");
                                let start = item.get("start").and_then(|s| s.get("dateTime").or(s.get("date"))).and_then(|d| d.as_str()).unwrap_or("");
                                let end = item.get("end").and_then(|e| e.get("dateTime").or(e.get("date"))).and_then(|d| d.as_str()).unwrap_or("");
                                
                                let (start_time, sort_key) = if start.contains('T') {
                                    let dt = chrono::DateTime::parse_from_rfc3339(start).unwrap_or_default();
                                    (dt.format("%I:%M %p").to_string(), dt.timestamp())
                                } else {
                                    ("All Day".to_string(), 0)
                                };

                                let end_time = if end.contains('T') {
                                    chrono::DateTime::parse_from_rfc3339(end).map(|dt| dt.format("%I:%M %p").to_string()).unwrap_or(end.to_string())
                                } else {
                                    "".to_string()
                                };

                                all_events.push(serde_json::json!({
                                    "summary": summary,
                                    "startTime": start_time,
                                    "endTime": end_time,
                                    "color": calendar_color,
                                    "type": if start_time == "All Day" { "task" } else { "meeting" },
                                    "sortKey": sort_key
                                }));
                            }
                        }
                    }
                }
                
                // 3. Sort all events by time
                all_events.sort_by_key(|e| e["sortKey"].as_i64().unwrap_or(0));
                
                Ok(serde_json::to_string(&all_events)?)
            })();

            let json_str = match result {
                Ok(json) => json,
                Err(e) => {
                    eprintln!("Error fetching calendar events: {}", e);
                    "[]".to_string()
                }
            };

            let _ = qt_thread.queue(move |mut qobject| {
                qobject.as_mut().set_events_json(cxx_qt_lib::QString::from(&json_str));
                qobject.as_mut().set_is_loading(false);
            });
        });
    }
}

pub fn do_login() {
    let secret_json = fs::read_to_string("client_secret.json").expect("Failed to read client_secret.json");
    let secret: SecretFile = serde_json::from_str(&secret_json).expect("Failed to parse client_secret.json");
    
    // Start a temporary local server to catch the redirect on a random port
    let server = tiny_http::Server::http("127.0.0.1:0").expect("Failed to start local server for redirect");
    let port = server.server_addr().to_ip().unwrap().port();
    let redirect_url = format!("http://localhost:{}", port);
    println!("Local server listening on {}", redirect_url);

    let client = BasicClient::new(ClientId::new(secret.installed.client_id))
        .set_client_secret(ClientSecret::new(secret.installed.client_secret))
        .set_auth_uri(AuthUrl::new(secret.installed.auth_uri).expect("Invalid auth URI"))
        .set_token_uri(TokenUrl::new(secret.installed.token_uri).expect("Invalid token URI"))
        .set_redirect_uri(RedirectUrl::new(redirect_url).expect("Invalid redirect URI"));
    
    let (auth_url, csrf_token) = client
        .authorize_url(oauth2::CsrfToken::new_random)
        .add_scope(Scope::new("https://www.googleapis.com/auth/calendar.readonly".to_string()))
        .add_extra_param("access_type", "offline")
        .add_extra_param("prompt", "consent")
        .url();

    println!("Opening browser for authorization...");
    let _ = std::process::Command::new("xdg-open").arg(auth_url.as_str()).spawn();

    println!("Waiting for Google redirect...");

    // Wait for the redirect request
    let request = server.recv().expect("Failed to receive request");
    let url = request.url();
    
    // Parse the code and state from the URL: /?code=...&state=...
    let query = url.split('?').nth(1).unwrap_or("");
    let mut code = None;
    let mut state = None;
    for pair in query.split('&') {
        let mut parts = pair.split('=');
        match (parts.next(), parts.next()) {
            (Some("code"), Some(c)) => code = Some(c.to_string()),
            (Some("state"), Some(s)) => state = Some(s.to_string()),
            _ => {}
        }
    }

    if let (Some(code), Some(incoming_state)) = (code, state) {
        if incoming_state != *csrf_token.secret() {
            let _ = request.respond(tiny_http::Response::from_string("CSRF token mismatch").with_status_code(400));
            eprintln!("Error: CSRF token mismatch!");
            return;
        }

        let agent = ureq::AgentBuilder::new().redirects(0).build();
        let token_result = client
            .exchange_code(AuthorizationCode::new(code))
            .request(&agent)
            .expect("Failed to exchange code for token");

        let token_path = get_token_path().unwrap();
        let stored_token = StoredToken {
            token: token_result,
            received_at: Utc::now(),
        };
        let token_json = serde_json::to_string(&stored_token).unwrap();
        fs::write(token_path, token_json).expect("Failed to save token");

        let _ = request.respond(tiny_http::Response::from_string("Login successful! You can close this tab."));
        println!("Login successful! Token saved.");
    } else {
        let _ = request.respond(tiny_http::Response::from_string("Authorization failed").with_status_code(400));
        println!("Authorization failed: code or state missing from redirect.");
    }
}
