use chrono::prelude::*;
use chrono::Duration;

fn main() {
    let now = Utc::now();
    // Display format is a human-readable format
    println!("Display format: {}", now);
    let iso = &now.to_rfc3339();
    println!("As ISO8601/RFC3339 with default settings: {}", iso);
    let iso = &now.to_rfc3339_opts(SecondsFormat::Secs, true);
    println!("As ISO8601/RFC3339 without nanoseconds: {}", iso);
    println!("Converted into UNIX timestamp: {}", now.timestamp());

    let local = Local::now();
    let offset = local.offset();
    println!("Current TZ offset: {}", offset);
    let iso_string = "2005-04-02T19:37:00Z";
    let parsed = DateTime::parse_from_rfc3339(&iso_string).unwrap();
    println!("Parsed into UTC: {}", parsed);
    let converted: DateTime<Local> = DateTime::from(parsed);
    println!("Converted into local time: {}", converted);
    let as_unix = converted.timestamp();
    println!("Converted into UNIX timestamp: {}", as_unix);

    // Parsing into local time
    let local: DateTime<Local> = DateTime::parse_from_rfc3339(&iso_string).unwrap().into();
    println!("Parsed directly as local time: {}", local);

    // DateTime from UNIX timestamp
    let timestamp = Local::now().timestamp();
    let converted = NaiveDateTime::from_timestamp(timestamp, 0);
    let converted = Local.from_utc_datetime(&converted);
    println!("{}", converted);

    // Adding and subtracting
    let now = Local::now();
    let two_weeks_from_now = now
        .checked_add_signed(Duration::weeks(2))
        .unwrap()
        .with_nanosecond(0)
        .unwrap();
    println!("{}", two_weeks_from_now);

    // Converting timezones
    let offset = FixedOffset::east(8 * 3600);
    let dt = now.with_timezone(&offset);
    println!("Taipei time: {}", dt);
}
