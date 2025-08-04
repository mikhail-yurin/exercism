#[derive(Debug)]
pub struct Duration(time::Duration);

impl Duration {
    pub fn as_secs(&self) -> u64 {
        self.0.whole_seconds() as u64
    }
}

impl From<u64> for Duration {
    fn from(sec: u64) -> Self {
        let duration = time::Duration::seconds(sec as i64);
        Duration(duration)
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn years_during(d: &Duration) -> f64 {
        const PERIOD: f64 = 0.2408467;
        round_to_places(d.as_secs() as f64 / 3600_f64 / 24_f64 / 365.25 / PERIOD, 2)
    }
}
impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        const PERIOD: f64 = 0.61519726;
        round_to_places(d.as_secs() as f64 / 3600_f64 / 24_f64 / 365.25 / PERIOD, 2)
    }
}
impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        const PERIOD: f64 = 1_f64;
        round_to_places(d.as_secs() as f64 / 3600_f64 / 24_f64 / 365.25 / PERIOD, 2)
    }
}
impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        const PERIOD: f64 = 1.8808158;
        round_to_places(d.as_secs() as f64 / 3600_f64 / 24_f64 / 365.25 / PERIOD, 2)
    }
}
impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        const PERIOD: f64 = 11.862615;
        round_to_places(d.as_secs() as f64 / 3600_f64 / 24_f64 / 365.25 / PERIOD, 2)
    }
}
impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        const PERIOD: f64 = 29.447498;
        round_to_places(d.as_secs() as f64 / 3600_f64 / 24_f64 / 365.25 / PERIOD, 2)
    }
}
impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        const PERIOD: f64 = 84.016846;
        round_to_places(d.as_secs() as f64 / 3600_f64 / 24_f64 / 365.25 / PERIOD, 2)
    }
}
impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        const PERIOD: f64 = 164.79132;
        round_to_places(d.as_secs() as f64 / 3600_f64 / 24_f64 / 365.25 / PERIOD, 2)
    }
}

fn round_to_places(value: f64, places: u32) -> f64 {
    let factor = 10_f64.powi(places as i32);
    (value * factor).round() / factor
}
