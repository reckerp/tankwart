use std::env;

pub struct Threshold {
    pub e5: Option<f64>,
    pub e10: Option<f64>,
    pub diesel: Option<f64>,
}

impl Threshold {
    pub fn from_env() -> Self {
        fn get_f64(key: &str) -> Option<f64> {
            env::var(key).ok()?.parse().ok()
        }

        Self {
            e5: get_f64("THRESHOLD_E5"),
            e10: get_f64("THRESHOLD_E10"),
            diesel: get_f64("THRESHOLD_DIESEL"),
        }
    }
}
