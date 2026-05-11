fn main() {
    let start = "2026-05-11T10:00:00Z";
    let dt = chrono::DateTime::parse_from_rfc3339(start).unwrap_or_default();
    println!("{}", dt.format("%I:%M %p").to_string());
}
