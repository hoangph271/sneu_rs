use chrono::{DateTime, NaiveDate, Utc};
use gloo_timers::callback::Interval;
use yew::{use_effect, use_state_eq};

const UPDATE_INTERVAL: u32 = 1000;
const IN_MS_30_DAYS: i64 = 30 * 24 * 60 * 60 * 1000;

fn get_d_day() -> DateTime<Utc> {
    let d_day = NaiveDate::from_ymd(2022, 9, 4).and_hms(16, 50, 0);

    DateTime::<Utc>::from_utc(d_day, Utc)
}

fn get_lasted() -> String {
    let today = Utc::now();
    let diff = today.signed_duration_since(get_d_day());

    if diff.num_days() > 1 {
        format!("{} days", diff.num_days())
    } else {
        format!("{} day", diff.num_days())
    }
}

fn get_progress() -> String {
    let today = Utc::now();
    let diff = today.signed_duration_since(get_d_day()).num_milliseconds() as f64;

    format!("{:.4}", (diff / IN_MS_30_DAYS as f64) * 100.0)
}

pub fn use_lasted() -> (String, String) {
    let lasted = use_state_eq(get_lasted);
    let progress = use_state_eq(get_progress);

    use_effect({
        let lasted = lasted.clone();
        let progress = progress.clone();

        move || {
            let timer = Interval::new(UPDATE_INTERVAL, move || {
                lasted.set(get_lasted());
                progress.set(get_progress());
            });

            || {
                timer.cancel();
            }
        }
    });

    ((*lasted).clone(), (*progress).clone())
}
