use chrono::{DateTime, NaiveDateTime, Utc};

pub fn to_date_str(date: &DateTime<Utc>) -> String {
    date.format("%Y-%m-%dT%H:%M").to_string()
}

pub fn from_date_str(date_str: &String) -> DateTime<Utc> {
    DateTime::<Utc>::from(
        NaiveDateTime::parse_from_str(date_str, "%Y-%m-%dT%H:%M")
            .unwrap_or_else(|e| {
                log::error!("Parse {date_str} failed: {e}");
                panic!()
            })
            .and_local_timezone(Utc)
            .unwrap(),
    )
}
