use chrono::{DateTime, Duration, Utc};
use gloo_timers::callback::Interval;
use yew::{use_effect, use_state_eq};

const UPDATE_INTERVAL: u32 = 1000;

fn get_diffs(started_at: &DateTime<Utc>, end_at: &DateTime<Utc>) -> (Duration, Duration) {
    let mut so_far = Utc::now().signed_duration_since(*started_at);
    if so_far.num_milliseconds() < 0 {
        so_far = Duration::zero();
    }

    let total = end_at.signed_duration_since(*started_at);

    (so_far, total)
}
fn is_done(started_at: &DateTime<Utc>, end_at: &DateTime<Utc>) -> bool {
    let (so_far, total) = get_diffs(started_at, end_at);

    so_far >= total
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

    if so_far.is_zero() {
        return "0".to_owned();
    }

    if so_far >= total {
        return "100".to_string();
    }

    let so_far = so_far.num_milliseconds() as f64;
    let total = total.num_milliseconds() as f64;

    format!("{:.2}", (so_far / total) * 100.0)
}

pub fn use_lasted(
    started_at: &DateTime<Utc>,
    end_at: &DateTime<Utc>,
) -> (String, String, bool, bool) {
    let lasted = use_state_eq(|| get_lasted(started_at, end_at));
    let progress = use_state_eq(|| get_progress(started_at, end_at));
    let is_done_state = use_state_eq(|| false);

    use_effect({
        let lasted = lasted.clone();
        let progress = progress.clone();
        let is_done_state = is_done_state.clone();
        let (started_at, end_at) = (*started_at, *end_at);

        move || {
            let timer = Interval::new(UPDATE_INTERVAL, move || {
                lasted.set(get_lasted(&started_at, &end_at));
                progress.set(get_progress(&started_at, &end_at));
                is_done_state.set(is_done(&started_at, &end_at));
            });

            || {
                timer.cancel();
            }
        }
    });

    let is_started = started_at.cmp(&Utc::now()).is_lt();

    (
        (*lasted).clone(),
        (*progress).clone(),
        *is_done_state,
        is_started,
    )
}
