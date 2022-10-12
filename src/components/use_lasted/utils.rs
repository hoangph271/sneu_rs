use chrono::{DateTime, Duration, Utc};
use gloo_timers::callback::Interval;
use yew::{use_effect, use_state_eq};

const UPDATE_INTERVAL: u32 = 1000;

fn get_diffs(started_at: &DateTime<Utc>, end_at: &DateTime<Utc>) -> (Duration, Duration) {
    let so_far = Utc::now().signed_duration_since(*started_at);
    let total = end_at.signed_duration_since(*started_at);

    (so_far, total)
}

fn get_lasted(started_at: &DateTime<Utc>, end_at: &DateTime<Utc>) -> String {
    let (so_far, total) = get_diffs(started_at, end_at);

    if so_far > total {
        return format!("{} days", total.num_days());
    }

    if so_far.num_days() > 1 {
        format!("{} days", so_far.num_days())
    } else {
        format!("{} day", so_far.num_days())
    }
}

fn get_progress(started_at: &DateTime<Utc>, end_at: &DateTime<Utc>) -> String {
    let (so_far, total) = get_diffs(started_at, end_at);

    if so_far >= total {
        return "100".to_string();
    }

    let so_far = so_far.num_milliseconds() as f64;
    let total = total.num_milliseconds() as f64;

    format!("{:.4}", (total / so_far) * 100.0)
}

pub fn use_lasted(started_at: &DateTime<Utc>, end_at: &DateTime<Utc>) -> (String, String) {
    let lasted = use_state_eq(|| get_lasted(started_at, end_at));
    let progress = use_state_eq(|| get_progress(started_at, end_at));

    use_effect({
        let lasted = lasted.clone();
        let progress = progress.clone();
        let (started_at, end_at) = (started_at.clone(), end_at.clone());

        move || {
            let timer = Interval::new(UPDATE_INTERVAL, move || {
                lasted.set(get_lasted(&started_at, &end_at));
                progress.set(get_progress(&started_at, &end_at));
            });

            || {
                timer.cancel();
            }
        }
    });

    ((*lasted).clone(), (*progress).clone())
}
