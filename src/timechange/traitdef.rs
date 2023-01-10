//use chrono::{DateTime, Local};
use time::OffsetDateTime;

pub struct CacheTimeInfo {
	pub sunrise: OffsetDateTime,
	pub sunset: OffsetDateTime,
	pub cacheday: u8,
}

impl CacheTimeInfo {
	pub fn new(sunrise: OffsetDateTime, sunset: OffsetDateTime, cacheday: u8) -> Self { Self { sunrise, sunset, cacheday } }
}
