mod traitdef;
use super::changeops;
use super::file::file;
use super::traitdef::AppOpt;
use anyhow::Result;
use chrono::{DateTime, Datelike, Local, TimeZone};
use std::{thread::sleep, time::Duration};
use traitdef::CacheTimeInfo;

pub fn timebased(lat: f64, long: f64, daybg: String, nightbg: String, appopt: AppOpt) -> Result<()> {
	let daybg = file(daybg)?;
	let nightbg = file(nightbg)?;
	let curtime = chrono::Local::now();
	let mut cache = gen_suntime(lat, long, curtime);
	let mut is_day = curtime > cache.sunrise || curtime < cache.sunset;

	if !is_day {
		changeops(daybg.clone(), &appopt)?;
	} else {
		changeops(nightbg.clone(), &appopt)?;
	}

	loop {
		sleep(Duration::from_secs(60));
		let curtime = chrono::Local::now();

		if cache.cacheday != curtime.day() {
			cache = gen_suntime(lat, long, curtime);
		}

		if (is_day && curtime > cache.sunrise && curtime < cache.sunset) || (!is_day && (curtime < cache.sunrise || curtime > cache.sunset) ) {
			continue;
		}

		if !is_day {
			changeops(daybg.clone(), &appopt)?;
			is_day = true;
		} else {
			changeops(nightbg.clone(), &appopt)?;
			is_day = false;
		}
	}
}

fn gen_suntime(latitude: f64, longitude: f64, curtime: DateTime<Local>) -> CacheTimeInfo {
	let (sunrise, sunset) = sunrise::sunrise_sunset(
		latitude,
		longitude,
		curtime.year(),
		curtime.month(),
		curtime.day(),
	);
	CacheTimeInfo::new(Local.timestamp(sunrise, 0), Local.timestamp(sunset, 0), curtime.day())
}
