use chrono::naive::NaiveDate;
use chrono::Local;

pub fn get_version() -> String {
    let birthdate = NaiveDate::from_ymd(1991, 8, 3);

    let age = (Local::today()
        .naive_local()
        .signed_duration_since(birthdate)
        .num_days()
        / 365) as f32;

    let minor = (age * 0.1).floor();
    let patch = (age * 0.1).fract();

    return format!("v1.{}.{}", minor, patch);
}
