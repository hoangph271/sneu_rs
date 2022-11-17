use chrono::{DateTime, NaiveDateTime, Utc};

pub fn friendly_datetime(date: &DateTime<Utc>) -> String {
    date.format("%Y/%m/%d %H:%M").to_string()
}

pub fn to_datetime_str(date: &DateTime<Utc>) -> String {
    date.format("%Y-%m-%dT%H:%M").to_string()
}

pub fn from_datetime_str(date_str: &String) -> DateTime<Utc> {
    NaiveDateTime::parse_from_str(date_str, "%Y-%m-%dT%H:%M")
        .unwrap_or_else(|e| {
            log::error!("Parse {date_str} failed: {e}");
            panic!()
        })
        .and_local_timezone(Utc)
        .unwrap()
}
