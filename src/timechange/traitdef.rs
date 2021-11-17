use chrono::{DateTime, Local};

pub struct CacheTimeInfo {
	pub sunrise: DateTime<Local>,
	pub sunset: DateTime<Local>,
	pub cacheday: u32,
}

impl CacheTimeInfo {
	pub fn new(sunrise: DateTime<Local>, sunset: DateTime<Local>, cacheday: u32) -> Self { Self { sunrise, sunset, cacheday } }
}
