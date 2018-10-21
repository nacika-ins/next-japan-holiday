extern crate chrono;
use chrono::prelude::*;
extern crate reqwest;
extern crate serde_json;
use serde_json::{Value};

#[derive(Debug)]
pub struct JapanHoliday {
    pub date: String,
    pub name: String,
    pub diff: i64
}

pub fn get_next_holiday() -> Result<JapanHoliday, String> {
    let api_url = "https://holidays-jp.github.io/api/v1/date.json";
    let mut res = reqwest::get(api_url).map_err(|e| format!("error: {}", e.to_string()) )?;
    let body = res.text().map_err(|e| format!("error: {}", e.to_string()) )?;
    let value: Value = serde_json::from_str(&body).map_err(|e| format!("error: {}", e.to_string()) )?;
    let holidays = value.as_object().ok_or("error".to_owned())?;
    let local_time = Local::now();
    let found_holiday: Vec<JapanHoliday> = holidays.iter().filter( |h| {
        let (date, _) = h;
        let date = DateTime::parse_from_str(&format!("{}  00:00:00 +09:00", date), "%Y-%m-%d %H:%M:%S %z");
        let bool = match date {
            Ok(date) => {
                let duration = local_time.signed_duration_since(date);
                duration.num_days() < 0i64
            }
            _ => false
        };
        bool
    } )
        .flat_map( |h| {

            let (date, name) = h;
            let date = DateTime::parse_from_str(&format!("{}  00:00:00 +09:00", date), "%Y-%m-%d %H:%M:%S %z");
            match date {
                Ok(date) => {
                    let duration = date.signed_duration_since(local_time);
                    Some(JapanHoliday { date: date.format("%Y年%m月%d日").to_string(), name: name.to_string(), diff: duration.num_days() + 1  })
                }
                _ => None
            }
        } )
        .collect();
    let first = found_holiday.first();
    match first {
        Some(ref v) => {Ok(
            JapanHoliday { date: v.date.clone(), name: v.name.clone(), diff: v.diff }
        )},
        _ => Err("error".to_owned())
    }
}

#[cfg(test)]
mod tests {
    use get_next_holiday;
    #[test]
    fn it_works() {
        let holiday = get_next_holiday();
        println!("{:?}", holiday);
        assert_eq!(2 + 2, 4);
    }
}

