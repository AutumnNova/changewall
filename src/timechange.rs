mod traitdef;
use super::stdoperation;
use super::traitdef::AppOpt;
use anyhow::Result;
use time::{OffsetDateTime, macros::offset};
use std::{thread::sleep};
use traitdef::CacheTimeInfo;

pub fn timebased(lat: f64, long: f64, daybg: &String, nightbg: &String, appopt: AppOpt) -> Result<()> {
	let mut curtime = OffsetDateTime::now_local().unwrap();
	let h = curtime.hour();
	if  h < 11 {
	curtime = curtime.replace_hour( (24 - (11 - h)) / 2).unwrap();
	} else {
	curtime = curtime.replace_hour(h - 11).unwrap();
	}
	//let curtime = OffsetDateTime::now_utc();
	//let curtime = chrono::Local::now();
	let mut cache = gen_suntime(lat, long, curtime);
	let mut day_state = cur_day(curtime.unix_timestamp(), cache.sunrise.unix_timestamp(), cache.sunset.unix_timestamp());
	println!("day is {}", day_state);
	if day_state {
		stdoperation(daybg, &appopt)?;
	} else {
		stdoperation(nightbg, &appopt)?;
	}
	println!("{} & {}", cache.sunrise.to_offset(offset!(+11)), cache.sunset.to_offset(offset!(+11)));

	loop {
		sleep(core::time::Duration::from_secs(5));
		let mut curtime = OffsetDateTime::now_local().unwrap();
		if  h < 11 {
			curtime = curtime.replace_hour( 24 - (11 - h)).unwrap();
			} else {
			curtime = curtime.replace_hour(h - 11).unwrap();
			}
		//let curtime = chrono::Local::now();
		println!("time: {:?}", curtime.to_hms());
		println!("time: {:?}", curtime.to_offset(offset!(UTC)).to_hms());
		if cache.cacheday != curtime.day() {
			cache = gen_suntime(lat, long, curtime);
		}

		if day_state == cur_day(curtime.unix_timestamp(), cache.sunrise.unix_timestamp(), cache.sunset.unix_timestamp()) {
			continue;
		} else {
			match day_state {
				true => {stdoperation(nightbg, &appopt)?; day_state = false;},
				false => {stdoperation(daybg, &appopt)?; day_state = true;},
			}
		}
	
	}
}

fn cur_day(curtime: i64, sunrise: i64, sunset: i64) -> bool {
	println!("{} {}", curtime < sunrise, curtime > sunset);
	!(curtime < sunrise && curtime > sunset)
}

fn gen_suntime(latitude: f64, longitude: f64, curtime: OffsetDateTime) -> CacheTimeInfo {
	let (sunrise, sunset) = sunrise::sunrise_sunset(
		latitude,
		longitude,
		curtime.year(),
		curtime.month() as u32,
		curtime.day().into(),
	);
	CacheTimeInfo::new(OffsetDateTime::from_unix_timestamp(sunrise).unwrap(), OffsetDateTime::from_unix_timestamp(sunset).unwrap(), curtime.day())
}
